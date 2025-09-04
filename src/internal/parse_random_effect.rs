use crate::internal::{ast::*, errors::ParseError, lexer::Token};

/// Parses a random effect term in the format (terms | grouping)
/// Supports various random effects syntax including:
/// - (1 | group) - Random intercepts
/// - (x | group) - Random slopes with intercepts
/// - (x || group) - Uncorrelated random effects
/// - (x |2| group) - Cross-parameter correlation
/// - (x | gr(group, cor = FALSE)) - Enhanced grouping
pub fn parse_random_effect<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<RandomEffect, ParseError> {
    // Expect opening parenthesis
    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionStart), "(")?;

    // Parse the terms (left side of |)
    let terms = parse_random_terms(tokens, pos)?;

    // Parse the correlation type and grouping (right side of |)
    let (correlation, correlation_id) = parse_correlation_type(tokens, pos)?;
    let grouping = parse_grouping(tokens, pos)?;

    // Expect closing parenthesis
    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;

    Ok(RandomEffect {
        terms,
        grouping,
        correlation,
        correlation_id,
    })
}

/// Parses the terms on the left side of the | in a random effect
fn parse_random_terms<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Vec<RandomTerm>, ParseError> {
    let mut terms = Vec::new();

    // Check for intercept suppression
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::One)) {
        // Check if followed by + or -
        if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
            // Parse additional terms
            while !crate::internal::matches::matches(tokens, pos, |t| {
                matches!(t, Token::Pipe | Token::DoublePipe)
            }) {
                terms.push(parse_random_term(tokens, pos)?);
                if !crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
                    break;
                }
            }
        } else {
            // Just intercept
            terms.push(RandomTerm::Column("1".to_string()));
        }
    } else if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Zero)) {
        // Check if followed by + (random slopes only)
        if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
            // Parse additional terms (no intercept)
            while !crate::internal::matches::matches(tokens, pos, |t| {
                matches!(t, Token::Pipe | Token::DoublePipe)
            }) {
                terms.push(parse_random_term(tokens, pos)?);
                if !crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
                    break;
                }
            }
        } else {
            // Just zero (no intercept) - but this should not happen in valid syntax
            // Zero should always be followed by + in random effects
            return Err(ParseError::Syntax(
                "expected '+' after '0' in random effects".into(),
            ));
        }
    } else if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Minus)) {
        // Check for -1 or -0 (intercept suppression)
        if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::One | Token::Zero))
        {
            terms.push(RandomTerm::SuppressIntercept);
        } else {
            return Err(ParseError::Syntax(
                "expected '1' or '0' after '-' for intercept suppression".into(),
            ));
        }
    } else {
        // Parse first term
        terms.push(parse_random_term(tokens, pos)?);

        // Parse additional terms
        while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
            terms.push(parse_random_term(tokens, pos)?);
        }
    }

    Ok(terms)
}

/// Parses a single random term (column, function, or interaction)
fn parse_random_term<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<RandomTerm, ParseError> {
    let (tok, name_slice) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| {
            matches!(
                t,
                Token::ColumnName | Token::FunctionStart | Token::Cs | Token::Mmc
            )
        },
        "ColumnName, FunctionStart, cs, or mmc",
    )?;

    match tok {
        Token::ColumnName => {
            // Check if this is followed by an interaction
            if crate::internal::matches::matches(tokens, pos, |t| {
                matches!(t, Token::InteractionOnly | Token::InteractionAndEffect)
            }) {
                let right_term = parse_random_term(tokens, pos)?;
                Ok(RandomTerm::Interaction {
                    left: Box::new(RandomTerm::Column(name_slice.to_string())),
                    right: Box::new(right_term),
                })
            } else {
                Ok(RandomTerm::Column(name_slice.to_string()))
            }
        }
        Token::FunctionStart => {
            // This should be handled by the main parser, not here
            Err(ParseError::Syntax(
                "unexpected function start in random term".into(),
            ))
        }
        Token::Cs => {
            // Parse cs() function
            crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::FunctionStart),
                "(",
            )?;

            // Parse the argument (can be 1, 0, or a column name)
            let (arg_tok, arg_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::One | Token::Zero | Token::ColumnName),
                "1, 0, or ColumnName",
            )?;

            let arg = match arg_tok {
                Token::One => crate::internal::ast::Argument::Integer(1),
                Token::Zero => crate::internal::ast::Argument::Integer(0),
                _ => crate::internal::ast::Argument::Ident(arg_str.to_string()),
            };

            crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;
            Ok(RandomTerm::Function {
                name: "cs".to_string(),
                args: vec![arg],
            })
        }
        Token::Mmc => {
            // Parse mmc() function
            crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::FunctionStart),
                "(",
            )?;
            let mut args = Vec::new();

            // Parse first argument
            let (_, arg_name) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::ColumnName),
                "ColumnName",
            )?;
            args.push(crate::internal::ast::Argument::Ident(arg_name.to_string()));

            // Parse additional arguments
            while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
                let (_, arg_name) = crate::internal::expect::expect(
                    tokens,
                    pos,
                    |t| matches!(t, Token::ColumnName),
                    "ColumnName",
                )?;
                args.push(crate::internal::ast::Argument::Ident(arg_name.to_string()));
            }

            crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;
            Ok(RandomTerm::Function {
                name: "mmc".to_string(),
                args,
            })
        }
        _ => Err(ParseError::Unexpected {
            expected: "random term",
            found: Some(tok),
        }),
    }
}

/// Parses the correlation type (|, ||, or |ID|)
fn parse_correlation_type<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<(CorrelationType, Option<String>), ParseError> {
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::DoublePipe)) {
        Ok((CorrelationType::Uncorrelated, None))
    } else if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Pipe)) {
        // Check for cross-parameter correlation ID
        if let Some((Token::Integer, id_slice)) = tokens.get(*pos) {
            *pos += 1;
            if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Pipe)) {
                Ok((
                    CorrelationType::CrossParameter(id_slice.to_string()),
                    Some(id_slice.to_string()),
                ))
            } else {
                return Err(ParseError::Syntax(
                    "expected second '|' after correlation ID".into(),
                ));
            }
        } else {
            Ok((CorrelationType::Correlated, None))
        }
    } else {
        Err(ParseError::Unexpected {
            expected: "| or ||",
            found: tokens.get(*pos).map(|(t, _)| t.clone()),
        })
    }
}

/// Parses the grouping structure (right side of |)
fn parse_grouping<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Grouping, ParseError> {
    let (tok, name_slice) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::ColumnName | Token::Gr | Token::Mm),
        "ColumnName, gr, or mm",
    )?;

    match tok {
        Token::ColumnName => {
            // Check for nested or interaction grouping
            if crate::internal::matches::matches(tokens, pos, |t| {
                matches!(t, Token::InteractionOnly)
            }) {
                let (_, right_name) = crate::internal::expect::expect(
                    tokens,
                    pos,
                    |t| matches!(t, Token::ColumnName),
                    "ColumnName",
                )?;
                Ok(Grouping::Interaction {
                    left: name_slice.to_string(),
                    right: right_name.to_string(),
                })
            } else if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Slash))
            {
                let (_, right_name) = crate::internal::expect::expect(
                    tokens,
                    pos,
                    |t| matches!(t, Token::ColumnName),
                    "ColumnName",
                )?;
                Ok(Grouping::Nested {
                    outer: name_slice.to_string(),
                    inner: right_name.to_string(),
                })
            } else {
                Ok(Grouping::Simple(name_slice.to_string()))
            }
        }
        Token::Gr => parse_gr_grouping(tokens, pos, name_slice),
        Token::Mm => parse_mm_grouping(tokens, pos),
        _ => Err(ParseError::Unexpected {
            expected: "grouping",
            found: Some(tok),
        }),
    }
}

/// Parses gr() grouping function
fn parse_gr_grouping<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
    _name_slice: &'a str,
) -> Result<Grouping, ParseError> {
    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionStart), "(")?;

    let (_, group_name) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::ColumnName),
        "ColumnName",
    )?;

    let mut options = Vec::new();

    // Parse options if present
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
        while !crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::FunctionEnd)) {
            options.push(parse_gr_option(tokens, pos)?);
            if !crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
                break;
            }
        }
    }

    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;

    Ok(Grouping::Gr {
        group: group_name.to_string(),
        options,
    })
}

/// Parses gr() function options
fn parse_gr_option<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<GrOption, ParseError> {
    let (tok, _name_slice) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| {
            matches!(
                t,
                Token::Cor | Token::Id | Token::By | Token::Cov | Token::Dist
            )
        },
        "gr option",
    )?;

    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::Equal), "=")?;

    match tok {
        Token::Cor => {
            let (value_tok, _value_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| {
                    matches!(
                        t,
                        Token::True | Token::TrueUpper | Token::False | Token::FalseUpper
                    )
                },
                "true or false",
            )?;
            Ok(GrOption::Cor(matches!(
                value_tok,
                Token::True | Token::TrueUpper
            )))
        }
        Token::Id => {
            let (value_tok, value_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::ColumnName | Token::StringLiteral),
                "ID string",
            )?;
            let id_value = match value_tok {
                Token::StringLiteral => value_str.trim_matches('"').to_string(),
                _ => value_str.to_string(),
            };
            Ok(GrOption::Id(id_value))
        }
        Token::By => {
            let (value_tok, value_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::ColumnName | Token::Null | Token::NullUpper),
                "by variable or NULL",
            )?;
            let by_value = match value_tok {
                Token::Null | Token::NullUpper => None,
                _ => Some(value_str.to_string()),
            };
            Ok(GrOption::By(by_value))
        }
        Token::Cov => {
            let (value_tok, _value_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| {
                    matches!(
                        t,
                        Token::True | Token::TrueUpper | Token::False | Token::FalseUpper
                    )
                },
                "true or false",
            )?;
            Ok(GrOption::Cov(matches!(
                value_tok,
                Token::True | Token::TrueUpper
            )))
        }
        Token::Dist => {
            let (value_tok, value_str) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::ColumnName | Token::StringLiteral),
                "distribution",
            )?;
            let dist_value = match value_tok {
                Token::StringLiteral => value_str.trim_matches('"').to_string(),
                _ => value_str.to_string(),
            };
            Ok(GrOption::Dist(dist_value))
        }
        _ => Err(ParseError::Unexpected {
            expected: "gr option",
            found: Some(tok),
        }),
    }
}

/// Parses mm() multi-membership grouping function
fn parse_mm_grouping<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Grouping, ParseError> {
    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionStart), "(")?;

    let mut groups = Vec::new();

    // Parse first group
    let (_, group_name) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::ColumnName),
        "ColumnName",
    )?;
    groups.push(group_name.to_string());

    // Parse additional groups
    while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
        let (_, group_name) = crate::internal::expect::expect(
            tokens,
            pos,
            |t| matches!(t, Token::ColumnName),
            "ColumnName",
        )?;
        groups.push(group_name.to_string());
    }

    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;

    Ok(Grouping::Mm { groups })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_simple_random_effect() {
        let tokens = vec![
            (Token::FunctionStart, "("),
            (Token::One, "1"),
            (Token::Pipe, "|"),
            (Token::ColumnName, "group"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_random_effect(&tokens, &mut pos);
        assert!(result.is_ok());
        let random_effect = result.unwrap();
        assert_eq!(random_effect.terms.len(), 1);
        assert!(matches!(random_effect.terms[0], RandomTerm::Column(ref name) if name == "1"));
        assert!(matches!(random_effect.grouping, Grouping::Simple(ref name) if name == "group"));
        assert!(matches!(
            random_effect.correlation,
            CorrelationType::Correlated
        ));
    }

    #[test]
    fn test_parse_uncorrelated_random_effect() {
        let tokens = vec![
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::DoublePipe, "||"),
            (Token::ColumnName, "group"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_random_effect(&tokens, &mut pos);
        assert!(result.is_ok());
        let random_effect = result.unwrap();
        assert_eq!(random_effect.terms.len(), 1);
        assert!(matches!(random_effect.terms[0], RandomTerm::Column(ref name) if name == "x"));
        assert!(matches!(
            random_effect.correlation,
            CorrelationType::Uncorrelated
        ));
    }

    #[test]
    fn test_parse_gr_grouping() {
        let tokens = vec![
            (Token::FunctionStart, "("),
            (Token::One, "1"),
            (Token::Pipe, "|"),
            (Token::Gr, "gr"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "group"),
            (Token::Comma, ","),
            (Token::Cor, "cor"),
            (Token::Equal, "="),
            (Token::False, "false"),
            (Token::FunctionEnd, ")"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_random_effect(&tokens, &mut pos);
        assert!(result.is_ok());
        let random_effect = result.unwrap();
        assert!(
            matches!(random_effect.grouping, Grouping::Gr { ref group, ref options } if group == "group" && options.len() == 1)
        );
    }
}
