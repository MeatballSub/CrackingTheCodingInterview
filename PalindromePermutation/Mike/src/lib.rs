use std::collections::HashMap;

pub mod test;

pub fn palindrome_permutation(string: &str) -> bool {
    if string.len() <= 1  {
        return true
    }

    let letters: Vec<_> = string.trim().to_lowercase().chars().filter(|c| !c.is_whitespace() && c.is_alphabetic()).collect();
    let identical = letters.iter().all(|c| *c == letters[0]);

    if identical {
        return true;
    }

    let mut letter_freq: HashMap<char, u32> = HashMap::new();
    let mut is_palindrome = false;


    letters.iter().for_each(|l| {
        *letter_freq.entry(*l).or_insert(0) += 1;
    });

    let mut uneven_frequency = 0;
    letter_freq.into_iter().for_each(|(key, val)| {
        if val % 2 != 0 {
           uneven_frequency += 1
        }
    });
   
    if uneven_frequency <= 1 {
        is_palindrome = true
    }

    is_palindrome
}


#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_palindrome_permutation() {
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
