use std::collections::HashSet;

pub mod test;

pub fn is_unique_hash_set(string: &str) -> bool {
    let mut unique_chars = HashSet::<char>::new();

    for character in string.chars() {
        if unique_chars.insert(character) == false {
            return false;
        }
    }

    true
}

pub fn is_unique_no_data_structures(string: &str) -> bool {
    for (i, character) in string.chars().enumerate() {
        let next_string = &string[(i + 1)..];
        for next_character in next_string.chars() {
            if character == next_character {
                return false;
            }
        }
    }

    true
}

pub fn is_unique(string: &str) -> bool {
    // is_unique_hash_set(string)
    is_unique_no_data_structures(string)
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
