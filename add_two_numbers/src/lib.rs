/// Adds two `i32` values and returns their sum.
///
/// # Examples
///
/// ```
/// assert_eq!(add_two_numbers::add_two_numbers(2, 3), 5);
/// assert_eq!(add_two_numbers::add_two_numbers(0, 33), 33);
/// ```
///
/// Negative values are supported:
///
/// ```
/// assert_eq!(add_two_numbers::add_two_numbers(-10, 3), -7);
/// ```
pub fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

pub fn subtract_two_integers(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add_two_numbers(0, 33), 33);
        assert_eq!(add_two_numbers(10, 20), 30);
    }

    #[test]
    fn adds_negative_and_mixed() {
        assert_eq!(add_two_numbers(-5, 5), 0);
        assert_eq!(add_two_numbers(-10, -3), -13);
    }

    #[test]
    fn subtracts_numbers() {
        assert_eq!(subtract_two_integers(5, 5), 0);
        assert_eq!(subtract_two_integers(5, -5), 10);
    }
}
