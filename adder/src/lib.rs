pub fn adder(a: u32, b: u32) -> u32 {
    if a > u32::MAX ^ b {
        panic!("Overflow occurred when adding {} and {}", a, b);
    }

    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;

    while carry != 0 {
        let temp = sum;
        sum = sum ^ carry;
        carry = (temp & carry) << 1;
    }
    sum
}

/*
 * Example with 15 + 27 = 42
 *
 * 15 = 0000 1111
 * 27 = 0001 1011
 *
 * Step 1:
 * 0000 1111 (15)
 * 0001 1011 (27)
 * -- XOR -- (^)
 * 0001 0100 (20)  <- sum without carry
 *
 * 0000 1111 (15)
 * 0001 1011 (27)
 * -- AND -- (&)
 * 0000 1011 (11)  <- carry
 * << 1
 * 0001 0110 (22)  <- carry shifted left
 *
 * Now we repeat until the carry is 0
 *
 * Iteration 1:
 * 0001 0100 (20)
 * 0001 0110 (22)
 * -- XOR -- (^)
 * 0000 0010 (2)  <- new sum without carry
 *
 * 0001 0100 (20)
 * 0001 0110 (22)
 * -- AND -- (&)
 * 0001 0100 (20)  <- carry bits
 * << 1
 * 0010 1000 (40)  <- carry shifted left
 *
 * Iteration 2:
 * 0000 0010 (2)
 * 0010 1000 (40)
 * -- XOR -- (^)
 * 0010 1010 (42)  <- new sum without carry
 *
 * 0000 0010 (2)
 * 0010 1000 (40)
 * -- AND -- (&)
 * 0000 0000 (0)  <- carry bits
 * << 1
 * 0000 0000 (0)  <- carry shifted left
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_adder() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(15, 27), 42);
        assert_eq!(adder(100, 200), 300);
    }

    #[test]
    fn bad_adder() {
        assert_ne!(adder(2, 2), 5);
        assert_ne!(adder(10, 20), 40);
    }

    #[test]
    #[should_panic(expected = "Overflow")]
    fn overflow_adder() {
        adder(u32::MAX, 1);
    }
}
