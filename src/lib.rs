//! A simple implementation of the `FizzBuzz` problem in Rust with the goal of being as short as possible.
//! More precisely, the implementation should fit into a single line of code when formatted with
//! `rustfmt` in its default configuration (<= 100 columns).

/// Returns a string representation of a number according to the following rules:
///
/// * Returns "fizz" if the number is divisible by `3`.
/// * Returns "buzz" if the number is divisible by `5`.
/// * Returns "fizzbuzz" if the number is divisible by both `3` and `5`.
/// * Returns the number as a string if it is not divisible by either `3` or `5`.
///
/// The implementation uses some tricks to fit the function into a single line of code:
///
/// * First, instead of branching, it indexes into an array of strings to determine the output.
/// * Additionally, instead of comparing the modulo to zero, it uses the `min()` function to determine
/// the correct index:
/// * `1.min(n % 3)` is equivalent to but shorter than `(n % 3 != 0) as usize`.
/// * `2.min(n % 5 * 2)` is equivalent to but shorter than `2 * (n % 5 != 0) as usize`.
///
/// # Arguments
///
/// * `n` - The number to be converted to a string according to the rules of the `FizzBuzz` problem.
///
/// # Examples
///
/// ```
/// # use fizz_buzz_golf::fizz_buzz;
/// assert_eq!(fizz_buzz(3), "fizz");
/// assert_eq!(fizz_buzz(5), "buzz");
/// assert_eq!(fizz_buzz(15), "fizzbuzz");
/// assert_eq!(fizz_buzz(4), "4");
/// ```
#[must_use]
pub fn fizz_buzz(n: usize) -> String {
    ["fizzbuzz", "buzz", "fizz", &n.to_string()][1.min(n % 3) + 2.min(n % 5 * 2)].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizz() {
        let expected = "fizz";

        assert_eq!(fizz_buzz(3), expected);
        assert_eq!(fizz_buzz(6), expected);
        assert_eq!(fizz_buzz(9), expected);
        assert_eq!(fizz_buzz(12), expected);
        assert_eq!(fizz_buzz(18), expected);
    }

    #[test]
    fn buzz() {
        let expected = "buzz";

        assert_eq!(fizz_buzz(5), expected);
        assert_eq!(fizz_buzz(10), expected);
        assert_eq!(fizz_buzz(20), expected);
        assert_eq!(fizz_buzz(25), expected);
        assert_eq!(fizz_buzz(35), expected);
    }

    #[test]
    fn fizzbuzz() {
        let expected = "fizzbuzz";

        assert_eq!(fizz_buzz(15), expected);
        assert_eq!(fizz_buzz(30), expected);
        assert_eq!(fizz_buzz(45), expected);
        assert_eq!(fizz_buzz(60), expected);
        assert_eq!(fizz_buzz(75), expected);
    }

    #[test]
    fn zero() {
        let expected = "fizzbuzz";

        assert_eq!(fizz_buzz(0), expected);
    }

    #[test]
    fn number() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
        assert_eq!(fizz_buzz(4), "4");
        assert_eq!(fizz_buzz(7), "7");
        assert_eq!(fizz_buzz(8), "8");
    }
}
