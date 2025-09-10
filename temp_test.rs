use fiasto::parse_formula; fn main() { let result = parse_formula("y ~ factor(treatment, ref=control) + c(group, ref=\"group1\")").unwrap(); println!("Both functions work together!"); }
