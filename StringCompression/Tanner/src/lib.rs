pub mod test;

pub fn string_compression(string: &str) -> String {
    todo!("Implement this function")
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
