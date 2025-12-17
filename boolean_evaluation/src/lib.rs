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

pub fn eval_formula(formula: &str) -> bool {
    if formula.is_empty() {
        panic!("Cannot eval an empty formula")
    }

    let tokens = tokenize(formula).expect("Failed to tokenize formula");
    let mut stack: Vec<bool> = Vec::new();

    for token in tokens {
        match token {
            OperationElt::Operator(operator) => {
                if stack.len() < 2 {
                    panic!("Unsufficient operands befor operator");
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
                        panic!("Unsufficient operands before NOT operator");
                    }
                    let value = stack.pop().expect("expected bool");
                    stack.push(!value);
                }
            },
        }
    }
    if stack.len() > 1 {
        panic!("Too many operands left on the stack");
    }

    stack.pop().expect("expected bool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_eval_formula() {
        assert_eq!(eval_formula("0!"), true);
        assert_eq!(eval_formula("1!"), false);

        assert_eq!(eval_formula("00|"), false);
        assert_eq!(eval_formula("10|"), true);
        assert_eq!(eval_formula("01|"), true);
        assert_eq!(eval_formula("11|"), true);

        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("11&"), true);

        assert_eq!(eval_formula("11^"), false);
        assert_eq!(eval_formula("10^"), true);

        assert_eq!(eval_formula("00>"), true);
        assert_eq!(eval_formula("01>"), true);
        assert_eq!(eval_formula("10>"), false);
        assert_eq!(eval_formula("11>"), true);

        assert_eq!(eval_formula("00="), true);
        assert_eq!(eval_formula("11="), true);
        assert_eq!(eval_formula("10="), false);
        assert_eq!(eval_formula("01="), false);

        assert_eq!(eval_formula("01&1|1="), true);
        assert_eq!(eval_formula("01&1&1&"), false);
        assert_eq!(eval_formula("0111&&&"), false);
    }

    #[test]
    #[should_panic(expected = "Unknown token")]
    fn wrong_parsing_eval_formula() {
        eval_formula("2&");
        eval_formula("(1)");
    }

    #[test]
    fn wrong_eval_formula() {
        assert_ne!(eval_formula("10&"), true);
        assert_ne!(eval_formula("11^"), true);
        assert_ne!(eval_formula("10>"), true);
        assert_ne!(eval_formula("01="), true);
    }

    #[test]
    #[should_panic(expected = "Cannot eval an empty formula")]
    fn empty_eval_formula() {
        eval_formula("");
    }

    #[test]
    #[should_panic(expected = "Unsufficient operands befor operator")]
    fn insufficient_operands_eval_formula() {
        eval_formula("1&");
        eval_formula("0&1|1=");
        eval_formula("01&1&&");
        eval_formula("0111&&");
    }
}
