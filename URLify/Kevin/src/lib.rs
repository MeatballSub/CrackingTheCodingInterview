pub mod test;

pub fn urlify(string: &Vec<char>, true_length: usize) -> Vec<char> {
    todo!("Implement this function")
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify() {
        let test_cases = read_test_cases();
        for ref test_case in test_cases {
            let actual = urlify(&test_case.input_string, test_case.input_true_length);
            assert_eq!(
                actual,
                test_case.answer,
                "input: '{},{}', expected: {}, actual: {}",
                String::from_iter(test_case.input_string.clone()),
                test_case.input_true_length,
                String::from_iter(test_case.answer.clone()),
                String::from_iter(actual.clone())
            );
        }
    }
}
