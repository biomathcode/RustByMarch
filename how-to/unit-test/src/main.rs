/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation.
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,  
/// which means you can just start writing code.
///
/// ```
/// let result = basic_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(10));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(is_even(3));
    }
}
