mod eval_formula;
mod truth_table;

use truth_table::print_truth_table;

fn main() {
    println!("A");
    print_truth_table("A");

    println!("AB&");
    print_truth_table("AB&");

    println!("AB>A>A>");
    print_truth_table("AB>A>A>");
}
