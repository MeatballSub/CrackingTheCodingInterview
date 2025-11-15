pub mod test;

fn get_varying_strings(string: &str) -> Vec<String> {
    let mut varying_strings: Vec<String> = Vec::new();

    let new_string = string.to_string();

    for (index, _character) in new_string.clone().chars().enumerate() {
        let mut varying_string = new_string.clone();
        varying_string.remove(index);
        varying_strings.push(varying_string);
    }

    varying_strings
}

fn is_different_lengths_one_away(larger_string: &str, smaller_string: &str) -> bool {
    let varying_strings = get_varying_strings(larger_string);

    for varying_string in varying_strings {
        if varying_string.eq(smaller_string) {
            return true;
        }
    }
    false
}

fn one_away_different_lengths(string1: &str, string2: &str) -> bool {
    let length_difference = string1.len().abs_diff(string2.len());

    if length_difference > 1 {
        return false;
    }

    let (larger_string, smaller_string) = if string1.len() > string2.len() {
        (string1, string2)
    } else {
        (string2, string1)
    };

    is_different_lengths_one_away(larger_string, smaller_string)
}

fn one_away_same_length(string1: &str, string2: &str) -> bool {
    let mut num_differences = 0;

    for (char1, char2) in string1.chars().zip(string2.chars()) {
        if !char1.eq(&char2) {
            num_differences += 1;
        }

        if num_differences > 1 {
            return false;
        }
    }

    true
}

pub fn one_away(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return one_away_different_lengths(string1, string2);
    }

    one_away_same_length(string1, string2)
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_one_away() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let actual = one_away(&test_case.string1, &test_case.string2);
            assert_eq!(
                actual, test_case.answer,
                "input: '{},{}', expected: {}, actual: {}",
                test_case.string1, test_case.string2, test_case.answer, actual
            );
        }
    }
}
