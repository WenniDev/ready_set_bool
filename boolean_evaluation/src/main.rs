use boolean_evaluation::eval_formula;

fn main() {
    println!("'0!' gives {}", eval_formula("0!"));
    println!("'1!' gives {}", eval_formula("1!"));
    println!("'00|' gives {}", eval_formula("00|"));
    println!("'10|' gives {}", eval_formula("10|"));
    println!("'01|' gives {}", eval_formula("01|"));
    println!("'11|' gives {}", eval_formula("11|"));

    println!("'11&1|1^' gives {}", eval_formula("11&1|1^"));
    println!("'0111&&&' gives {}", eval_formula("0111&&&"))
}
