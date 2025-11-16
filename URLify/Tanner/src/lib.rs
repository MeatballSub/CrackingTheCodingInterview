pub mod test;

fn shift_and_insert(string: &mut Vec<char>, index: usize) -> &Vec<char> {
    let end_index = string.len() - 1;

    for i in 0..(string.len() - 2) {
        let curr_index = end_index - i;
        let swap_index = curr_index - 2;

        if curr_index == index {
            break;
        }

        string[curr_index] = string[swap_index];
    }

    string[index] = '%';
    string[index + 1] = '2';
    string[index + 2] = '0';

    string
}

pub fn urlify(string: &mut Vec<char>, _true_length: usize) -> &Vec<char> {
    for i in 0..string.len() {
        let character = string[i];

        if character.is_whitespace() {
            shift_and_insert(string, i);
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
