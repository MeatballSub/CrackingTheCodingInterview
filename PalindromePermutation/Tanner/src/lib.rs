use std::collections::HashMap;

pub mod test;

pub fn palindrome_permutation(string: &str) -> bool {
    if string.is_empty() {
        return true;
    }

    let mut char_map: HashMap<char, usize> = HashMap::new();

    for character in string.to_lowercase().chars() {
        if !character.is_alphabetic() {
            continue;
        }
        *char_map.entry(character).or_insert(0) += 1;
    }

    let mut num_odds = 0;

    for num in char_map.values() {
        if num % 2 != 0 {
            num_odds += 1;
        }
    }

    num_odds % 2 != 0
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify() {
        let test_cases = read_test_cases();
        for ref test_case in test_cases {
            let actual = palindrome_permutation(&test_case.input);
            assert_eq!(
                actual, test_case.answer,
                "input: '{}', expected: {}, actual: {}",
                test_case.input, test_case.answer, actual
            );
        }
    }
}
