pub mod test;

// Implement an algorithm to determine if a string has all unique characters.
// What if you cannot use additional data structures?
pub fn is_unique(string: &str) -> bool {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_is_unique() {
        let test_cases = read_test_cases();
        for ref test_case in test_cases {
            let actual = is_unique(&test_case.input);
            assert_eq!(
                actual, test_case.answer,
                "input: '{}', expected: {}, actual: {}",
                test_case.input, test_case.answer, actual
            );
        }
    }
}
