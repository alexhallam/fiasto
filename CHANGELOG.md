# Changelog

## [Unreleased]

### ✨ Added

- **Identity Role for Plain Terms**: Added new `Identity` role to `VariableRole` enum for variables that appear as plain terms in formulas (e.g., `x` in `y ~ x`)
- **Intercept Column Support**: Added automatic inclusion of `"intercept"` column in `all_generated_columns` when `has_intercept` is true
- **Formula Order Mapping**: Added new `all_generated_columns_formula_order` field that maps formula order (1, 2, 3...) to column names
- **Intercept-Only Model Support**: Added support for intercept-only models like `y ~ 1` and no-intercept models like `y ~ 0` with new `Term::Intercept` and `Term::Zero` variants
- **Tests**: Added 7 new unit tests to verify intercept handling, formula order mapping, and intercept-only model functionality

### 🔧 Improved

- **Variable Role Assignment**: Plain terms now correctly receive `Identity` role instead of `FixedEffect` role
- **Generated Columns Preservation**: Variables with `Identity` role now preserve their original column name in generated columns list
- **Intercept Positioning**: Intercept column is automatically inserted at index 1 (after response variable) in `all_generated_columns`

### 🐛 Fixed

- **Issue #1**: Fixed intercept-only model parsing by adding support for `y ~ 1` and `y ~ 0` formulas
- **Issue #4**: Fixed plain terms not receiving proper `Identity` role when appearing alone in formulas
- **Issue #6**: Fixed missing intercept column in `all_generated_columns` and added formula order mapping

### 🧪 Testing

- **Unit Test Coverage**: Added comprehensive test coverage for intercept, formula order, and intercept-only model functionality
- **Regression Prevention**: Tests ensure intercept is present when `has_intercept` is true and absent when false
- **Order Validation**: Tests verify correct column ordering in both `all_generated_columns` and `all_generated_columns_formula_order`
- **Intercept-Only Models**: Tests verify `y ~ 1` and `y ~ 0` formulas work correctly with proper metadata generation
- **Error Handling**: Tests verify invalid syntax like `y ~ 1 - 1` and `y ~ 0 + 1` fails appropriately

### 🔄 Internal Changes

- **AST Enhancement**: Added new `Term::Intercept` and `Term::Zero` variants to support intercept-only and no-intercept models
- **Parser Updates**: Enhanced `parse_term()` to recognize `Token::One` and `Token::Zero` as valid terms
- **Syntax Validation**: Added validation to prevent contradictory syntax like `y ~ 1 - 1` and invalid combinations like `y ~ 0 + 1`
- **MetaBuilder Enhancement**: Updated `build()` method to handle intercept insertion and formula order mapping
- **Data Structure Updates**: Enhanced `FormulaMetaData` struct with new `all_generated_columns_formula_order` field
- **Role Management**: Improved role assignment logic in `push_plain_term()` and `add_transformation()` methods

## [0.2.4] - 2025-09-05

### Added
- Cleaned docs 

## [0.2.3] - 2025-09-05

### ✨ Added

- **Enhanced Error Messages**: Added colored, user-friendly error reporting with contextual information
  - Pretty-printed syntax errors show original formula, successful lexemes (green), and failed lexemes (red)
  - Clear "Expected Token:" labels with clean token names
  - Contextual display of parsing progress with colored highlighting using `owo-colors`
- **Improved Parser Error Context**: Parse errors now include the exact position and context where parsing failed

### 🔧 Improved

- **Error Message Quality**: Cleaned up error messages to show "Function or ColumnName" instead of "Function token or ColumnName"
- **Code Style**: Eliminated clippy warnings throughout the codebase
  - Removed unnecessary `to_string()` calls in format strings
  - Converted instance methods to static methods where appropriate
  - Removed unnecessary return statements
  - Fixed redundant pattern matching in examples

### 🐛 Fixed

- **Error Display**: Fixed duplicate and malformed error message formatting
- **Documentation**: Removed duplicate doc comments and improved consistency

### 📚 Documentation

- **Error Handling**: Updated parser documentation to reflect new pretty error functionality
- **Examples**: Added comprehensive error testing examples demonstrating colored output

### 🧹 Code Quality

- **Dead Code**: Eliminated all dead code warnings in main library
- **Clippy Compliance**: Achieved zero clippy warnings for main library code
- **Performance**: Optimized error formatting to avoid unnecessary string allocations

### 🔄 Internal Changes

- **Error Processing**: Streamlined error handling pipeline to preserve original `ParseError::Unexpected` for better formatting
- **String Processing**: Improved error message generation to be more efficient and readable

#### Example Output Format:
```
Syntax error- Unexpected Token
Formula: y ~ x +
Show: y ~ x + <eoi>
Expected Token: Function or ColumnName
```

These changes maintain backward compatibility while significantly improving the developer experience when working with formula parsing errors.

## [0.2.2] - 2025-09-05

added `lex_formula` for users to inspect raw lexer output

## [0.2.1] - 2025-09-04

just to trigger a new release

## [0.2.2] - 2025-09-05

### Added

- Multiplication/interaction parsing: Support for `*` (full interaction) in formulas was added so expressions like `wt*hp` are parsed correctly and represented as interaction terms in the AST and metadata output.

### Changed

- Parser internals: Improved term parsing and interaction handling to correctly parse chained interactions (`a*b*c`) and mixed interaction operators (`:` and `*`). The implementation centralises interaction handling to avoid double-consuming tokens and to make chaining robust.

- Files changed:
  - `src/internal/parse_term.rs` — refactored and documented to parse atomic terms (columns/functions), then build interaction chains by consuming `:` and `*` tokens and constructing `Term::Interaction` nodes.
  - `src/internal/parse_rhs.rs` — adjusted plus-separated term handling to avoid double token consumption when iterating `+`-separated terms.

### Added (debug)

- Temporary example `examples/print_tokens.rs` used to inspect lexer output while debugging interaction token ordering. This can be removed after verification.

### Notes

- The changes include extra inline documentation in the modified files. I ran the `examples/mtcars` example to validate behavior and confirmed the output now includes the `wt*hp` interaction and correct generated columns from `poly(disp, 4)`.


## [0.2.0] - 2025-09-04

### Added

- **Advanced Random Effects Syntax**: Complete brms-style random effects parsing including:
  - Random intercepts and slopes: `(1 | group)`, `(0 + x | group)`, `(-1 + x | group)`
  - Correlation types: `|` (correlated), `||` (uncorrelated), `|ID|` (cross-parameter)
  - Enhanced grouping with `gr()` function supporting `cor`, `id`, `by`, `cov`, `dist` arguments... kind of still WIP
  - Multi-membership structures: `mm()`, `mmc()`... WIP
  - Category-specific effects: `cs()`... WIP
  - Hierarchical/nested structures: `group1/group2` ... WIP
  - Interaction of grouping factors: `group1:group2` ... WIP
  - Suppression of random intercepts: `0 +` or `-1 +`
- **Variable-Centric Metadata Structure**: Complete redesign of output format:
  - Variables as first-class citizens with comprehensive attributes
  - Roles: Response, FixedEffect, GroupingVariable, RandomEffect
  - Transformations with generated column tracking
  - Interactions with proper naming
  - Random effects information per variable
- **Enhanced Function Support**: Added support for `log()` function and improved function parsing
- **Interaction Support**: Fixed effects interactions with `x:z` syntax
- **Family Information**: Always included in metadata output
- **Generated Columns Tracking**: Every variable tracks its generated columns
- **ID Ordering**: Response variable always gets ID 1, others start from ID 2
- **All Generated Columns Array**: Complete list of all generated columns ordered by ID
- **Comprehensive Documentation**: Added detailed `gr()` function documentation .. WIP

### Changed

- **Breaking Change**: Complete metadata structure redesign from effect-centric to variable-centric
- **Breaking Change**: New JSON output format with variables as primary entities
- **Breaking Change**: Version bump to 0.2.0 due to significant API changes

### Fixed

- Fixed grouping factor inclusion in random effects columns
- Fixed token ordering in lexer for proper keyword recognition
- Fixed interaction parsing in complex formulas
- Fixed hardcoded interaction naming

## [0.1.6] - 2025-09-03

### Added

- added more detailed doc output

## [0.1.5] - 2025-09-03

### Added

- spelling

## [0.1.4] - 2025-09-03

### Added

- new logo

## [0.1.3] - 2025-09-03

### Removed

- some toml keywords. only 5 allowed.

## [0.1.2] - 2025-09-03

### Added

- patch for release workflow

## [0.1.1] - 2025-09-03

### Added

- patch for release workflow

## [0.1.0] - 2025-09-03

### Added

- Initial release