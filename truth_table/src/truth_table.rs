use crate::eval_formula::eval_formula;

fn extract_variables(formula: &String) -> Vec<String> {
    let mut vars: Vec<String> = Vec::new();
    for ch in formula.chars() {
        let ch_str = ch.to_string();
        if ch.is_ascii_uppercase() && !vars.contains(&ch_str) {
            vars.push(ch_str);
        }
    }
    vars.sort();
    vars.dedup();
    vars
}

fn substitute_variables(formula: &String, values: &Vec<bool>, variables: &Vec<String>) -> String {
    let mut substituted = formula.clone();
    for (i, var) in variables.iter().enumerate() {
        let val_str = if values[i] { "1" } else { "0" };
        substituted = substituted.replace(var, val_str);
    }
    substituted
}

fn is_valid_parsing(formula: &String) -> Result<bool, String> {
    let variables = extract_variables(&formula);
    let mut test_formula = formula.clone();
    for var in variables {
        test_formula = test_formula.replace(&var, "1");
    }
    eval_formula(&test_formula)
}

fn print_row_str(values: &Vec<&str>, result: &str) {
    print!("|");
    for val in values {
        print!(" {} |", val);
    }
    println!(" {} |", result);
}

fn print_row_bool(values: &Vec<bool>, result: bool) {
    print!("|");
    for val in values {
        let val_str = if *val { "1" } else { "0" };
        print!(" {} |", val_str);
    }
    let result_str = if result { "1" } else { "0" };
    println!(" {} |", result_str);
}

fn print_header(variables: &Vec<String>) {
    print!("|");
    for _ in variables {
        print!("---|");
    }
    println!("---|");
}

pub fn print_truth_table(formula: &str) {
    if formula.is_empty() {
        panic!("Cannot print truth table for an empty formula")
    }

    is_valid_parsing(&formula.to_string()).expect("Error parsing formula");

    let variables = extract_variables(&formula.to_string());
    let var_count = variables.len();

    let var_refs: Vec<&str> = variables.iter().map(|s| s.as_str()).collect();
    print_row_str(&var_refs, "=");
    print_header(&variables);

    for i in 0..(2u32.pow(var_count as u32)) {
        let mut values: Vec<bool> = Vec::new();
        for j in 0..var_count {
            let value = (i >> (var_count - j - 1)) & 1 == 1;
            values.push(value);
        }

        let substituted_formula = substitute_variables(&formula.to_string(), &values, &variables);
        let result = eval_formula(&substituted_formula).expect("Error evaluating formula");

        print_row_bool(&values, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let formula = "ABCA&D>".to_string();
        let vars = extract_variables(&formula);
        assert_eq!(vars, vec!["A", "B", "C", "D"]);
    }

    #[test]
    fn test_substitute_variables() {
        let formula = "AB&C>".to_string();
        let variables = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let values = vec![true, false, true];
        let substituted = substitute_variables(&formula, &values, &variables);
        assert_eq!(substituted, "10&1>");
    }

    #[test]
    fn test_is_valid_parsing() {
        let formula = "AB&C>".to_string();
        let result = is_valid_parsing(&formula).unwrap();
        assert!(result);

        let invalid_formula = "AB&>".to_string();
        let result_invalid = is_valid_parsing(&invalid_formula);
        assert!(result_invalid.is_err());
    }

    #[test]
    fn test_print_truth_table() {
        print_truth_table("A");
        print_truth_table("AB&");
        print_truth_table("AB>A>A>");
    }
}
