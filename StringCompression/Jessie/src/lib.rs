pub mod test;

// Implement a method to perform basic string compression using the the counts
// of repeated characters.  For example, the string aabcccccaaa would become
// a2b1c5a3.  If the "compressed" string would not become smaller than the
// original string, your method should return the original string.  You can
// assume the string has only uppercase and lowercase letters(A-Za-z).
pub fn string_compression(string: &str) -> String {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_string_compression() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let actual = string_compression(&test_case.string);
            assert_eq!(
                actual, test_case.answer,
                "input: '{}', expected: {}, actual: {}",
                test_case.string, test_case.answer, actual
            );
        }
    }
}
