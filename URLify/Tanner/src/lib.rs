pub mod test;

pub fn urlify(string: &mut Vec<char>, true_length: usize) -> &Vec<char> {
    // remove trailing whitespace based on true length
    for _i in 0..(string.len() - true_length) {
        let removal_index = string.len() - 1;
        string.remove(removal_index);
    }

    // iterate through the string in reverse order
    // replace whitespace with "%20"
    for (index, character) in string.clone().iter().enumerate().rev() {
        if character.is_whitespace() {
            string.remove(index);
            string.insert(index, '0');
            string.insert(index, '2');
            string.insert(index, '%');
        }
    }

    string
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
