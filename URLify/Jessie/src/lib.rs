pub mod test;

// Write a method to replace all spaces in a string with '%20'.  You may assume
// that the string has sufficient space at the end to hold the additional
// characters, and that you are given the "true" length of the string.
// EXAMPLE:
// Input:  "Mr John Smith    ", 13
// Output: "Mr%20John%20Smith"
pub fn urlify(string: &mut Vec<char>, _true_length: usize) -> &Vec<char> {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let mut input_string = test_case.input_string.clone();
            let actual = urlify(&mut input_string, test_case.input_true_length);
            assert_eq!(
                *actual,
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
