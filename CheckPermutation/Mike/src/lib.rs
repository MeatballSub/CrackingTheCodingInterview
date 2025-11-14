use std::{collections::HashMap};

pub mod test;

fn calc_freq(string: &str) -> HashMap<char, u32> {
    let mut letter_freq: HashMap<char, u32> = HashMap::new();
    let letters: Vec<_> = string
        .trim()
        .to_lowercase()
        .chars()
        .collect();

    letters.iter().for_each(|l| {
        *letter_freq.entry(*l).or_insert(0) += 1;
    });

    letter_freq
}

pub fn check_permutation(string1: &str, string2: &str) -> bool {
    let string_1_vec: Vec<_> = string1.chars().collect();
    let string_2_vec: Vec<_> = string2.chars().collect();

    if string_1_vec.len() != string_2_vec.len() {
        return false
    }

    let mut s1_letter_freq: HashMap<char, u32> = calc_freq(string1);
    let mut s2_letter_freq: HashMap<char, u32> = calc_freq(string2);

    s1_letter_freq == s2_letter_freq
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
