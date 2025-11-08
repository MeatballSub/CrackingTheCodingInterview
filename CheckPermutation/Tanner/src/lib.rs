use std::collections::HashMap;

pub mod test;

pub fn check_permutation(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let mut characters1 = HashMap::new();
    let mut characters2 = HashMap::new();

    for (char1, char2) in string1.chars().zip(string2.chars()) {
        *characters1.entry(char1).or_insert(0) += 1;
        *characters2.entry(char2).or_insert(0) += 1;
    }

    for (key, value1) in characters1.iter() {
        let Some(value2) = characters2.get(key) else {
            return false;
        };

        if *value1 != *value2 {
            return false;
        }
    }

    return true;
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
