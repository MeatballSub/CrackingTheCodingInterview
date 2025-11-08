pub mod test;

pub fn check_permutation(string1: &str, string2: &str) -> bool {
    todo!("Implement this function")
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_check_permutation() {
        let test_cases = read_test_cases();
        for ref test_case in test_cases {
            let actual = check_permutation(&test_case.input1, &test_case.input2);
            assert_eq!(
                actual, test_case.answer,
                "input: '{},{}', expected: {}, actual: {}",
                test_case.input1, test_case.input2, test_case.answer, actual
            );
        }
    }
}
