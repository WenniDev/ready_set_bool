use adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut multiplicand = a;
    let mut multiplier = b;

    while multiplier > 0 {
        if (multiplier & 1) != 0 {
            result = adder(result, multiplicand);
        }

        if (multiplicand & (1 << 31)) != 0 && multiplier > 1 {
            panic!("Overflow occurred when multiplying {} and {}", a, b);
        }

        multiplicand <<= 1;
        multiplier >>= 1
    }

    result
}

/*
 * Example with 3 * 5 = 15
 *
 * 3 = 0000 0011
 * 5 = 0000 0101
 *
 * Iteration 1:
 * 0000 0101 (5)  <- multiplier
 * 0000 0001 (1)
 * -- AND -- (&)
 * 0000 0001 (1)  <- least significant bit is 1,
 *
 * so we add multiplicand (3) to result once
 * 0000 0011 (3) <- result
 *
 * 0000 0101 (5) <- multiplier
 * >> 1
 * 0000 0010 (2) <- shift right
 *
 * 0000 0011 (3) <- multiplicand
 * << 1
 * 0000 0110 (6) <- shift left
 *
 * Iteration 2:
 * 0000 0010 (2)  <- multiplier
 * 0000 0001 (1)
 * -- AND -- (&)
 * 0000 0000 (0)  <- least significant bit is 0,
 *
 * so we do not add
 *
 * 0000 0010 (2) <- multiplier
 * >> 1
 * 0000 0001 (1) <- shift right
 *
 * 0000 0110 (6) <- multiplicand
 * << 1
 * 0000 1100 (12) <- shift left
 *
 * Iteration 3:
 * 0000 0001 (1)  <- multiplier
 * 0000 0001 (1)
 * -- AND -- (&)
 * 0000 0001 (1)  <- least significant bit is 1,
 *
 * so we add multiplicand (12) to result once
 * 0000 0011 (3) + 0000 1100 (12) = 0000 1111 (15) <- result
 *
 * 0000 0001 (1) <- multiplier
 * >> 1
 * 0000 0000 (0) <- shift right
 *
 * 0000 1100 (12) <- multiplicand
 * << 1
 * 0011 0000 (24) <- shift left
 *
 */

#[cfg(test)]
mod test {
    use super::multiplier;

    #[test]
    fn good_multiplier() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(2, 3), 6);
        assert_eq!(multiplier(4, 5), 20);
        assert_eq!(multiplier(7, 8), 56);
    }

    #[test]
    fn bad_multiplier() {
        assert_ne!(multiplier(3, 3), 10);
        assert_ne!(multiplier(6, 7), 50);
    }

    #[test]
    #[should_panic(expected = "Overflow")]
    fn overflow_multiplier() {
        multiplier(u32::MAX / 2 + 1, 2);
    }
}
