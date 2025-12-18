enum Operator {
    AND,  // &
    OR,   // |
    XOR,  // ^
    IMPL, // > (implication)
    EQ,   // = (equivalence)
}

enum Operand {
    FALSE, // 0
    TRUE,  // 1
    NOT,   // !
}

enum OperationElt {
    Operator(Operator),
    Operand(Operand),
}

fn tokenize(expr: &str) -> Result<Vec<OperationElt>, String> {
    expr.chars()
        .map(|token| match token {
            '&' => Ok(OperationElt::Operator(Operator::AND)),
            '|' => Ok(OperationElt::Operator(Operator::OR)),
            '^' => Ok(OperationElt::Operator(Operator::XOR)),
            '>' => Ok(OperationElt::Operator(Operator::IMPL)),
            '=' => Ok(OperationElt::Operator(Operator::EQ)),
            '1' => Ok(OperationElt::Operand(Operand::TRUE)),
            '0' => Ok(OperationElt::Operand(Operand::FALSE)),
            '!' => Ok(OperationElt::Operand(Operand::NOT)),
            _ => Err(format!("Unknown token: {}", token)),
        })
        .collect()
}

pub fn eval_formula(formula: &str) -> Result<bool, String> {
    if formula.is_empty() {
        return Err("Cannot evaluate an empty formula".to_string());
    }

    let tokens = tokenize(formula).expect("Failed to tokenize formula");
    let mut stack: Vec<bool> = Vec::new();

    for token in tokens {
        match token {
            OperationElt::Operator(operator) => {
                if stack.len() < 2 {
                    return Err(format!("Unsufficient operands befor operator"));
                }

                let operand2 = stack.pop().expect("expected bool");
                let operand1 = stack.pop().expect("expected bool");

                let result = match operator {
                    Operator::AND => operand1 & operand2,
                    Operator::OR => operand1 | operand2,
                    Operator::XOR => operand1 ^ operand2,
                    Operator::IMPL => !operand1 | operand2,
                    Operator::EQ => operand1 == operand2,
                };
                stack.push(result);
            }
            OperationElt::Operand(operand) => match operand {
                Operand::TRUE => stack.push(true),
                Operand::FALSE => stack.push(false),
                Operand::NOT => {
                    if stack.is_empty() {
                        return Err(format!("Unsufficient operands before NOT operator"));
                    }
                    let value = stack.pop().expect("expected bool");
                    stack.push(!value);
                }
            },
        }
    }
    if stack.len() > 1 {
        return Err(format!("Too many operands left on the stack"));
    }

    Ok(stack.pop().expect("expected bool"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_eval_formula() {
        assert_eq!(eval_formula("0!").unwrap(), true);
        assert_eq!(eval_formula("1!").unwrap(), false);

        assert_eq!(eval_formula("00|").unwrap(), false);
        assert_eq!(eval_formula("10|").unwrap(), true);
        assert_eq!(eval_formula("01|").unwrap(), true);
        assert_eq!(eval_formula("11|").unwrap(), true);

        assert_eq!(eval_formula("10&").unwrap(), false);
        assert_eq!(eval_formula("11&").unwrap(), true);

        assert_eq!(eval_formula("11^").unwrap(), false);
        assert_eq!(eval_formula("10^").unwrap(), true);

        assert_eq!(eval_formula("00>").unwrap(), true);
        assert_eq!(eval_formula("01>").unwrap(), true);
        assert_eq!(eval_formula("10>").unwrap(), false);
        assert_eq!(eval_formula("11>").unwrap(), true);

        assert_eq!(eval_formula("00=").unwrap(), true);
        assert_eq!(eval_formula("11=").unwrap(), true);
        assert_eq!(eval_formula("10=").unwrap(), false);
        assert_eq!(eval_formula("01=").unwrap(), false);

        assert_eq!(eval_formula("01&1|1=").unwrap(), true);
        assert_eq!(eval_formula("01&1&1&").unwrap(), false);
        assert_eq!(eval_formula("0111&&&").unwrap(), false);
    }

    #[test]
    #[should_panic(expected = "Unknown token")]
    fn wrong_parsing_eval_formula() {
        eval_formula("2&").unwrap();
        eval_formula("(1)").unwrap();
    }

    #[test]
    fn wrong_eval_formula() {
        assert_ne!(eval_formula("10&").unwrap(), true);
        assert_ne!(eval_formula("11^").unwrap(), true);
        assert_ne!(eval_formula("10>").unwrap(), true);
        assert_ne!(eval_formula("01=").unwrap(), true);
    }

    #[test]
    fn empty_eval_formula() {
        assert_eq!(
            eval_formula(""),
            Err(format!("Cannot evaluate an empty formula"))
        );
    }

    #[test]
    fn insufficient_operands_eval_formula() {
        assert_eq!(
            eval_formula("1&"),
            Err(format!("Unsufficient operands befor operator"))
        );
        assert_eq!(
            eval_formula("0&1|1="),
            Err(format!("Unsufficient operands befor operator"))
        );
        assert_eq!(
            eval_formula("01&1&&"),
            Err(format!("Unsufficient operands befor operator"))
        );
        assert_eq!(
            eval_formula("!"),
            Err(format!("Unsufficient operands before NOT operator"))
        );
        assert_eq!(
            eval_formula("0111&&"),
            Err(format!("Too many operands left on the stack"))
        );
    }
}
