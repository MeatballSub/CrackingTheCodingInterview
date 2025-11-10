use std::collections::HashMap;

pub mod test;

pub fn palindrome_permutation(string: &str) -> bool { palindrome_permutation_kevin_3(string) }

pub fn palindrome_permutation_kevin_1(string: &str) -> bool
{
    let mut frequencies = [0usize; 26];
    let string_bytes = string.as_bytes();
    for ch in string_bytes
    {
        // 32 is the mask to set the bit to do a to lower
        let ch_index = ch | 32;
        if b'a' <= ch_index && ch_index <= b'z'
        {
            frequencies[(ch_index - b'a') as usize] += 1;
        }
    }

    frequencies.iter().filter(|c| *c % 2 == 1).count() <= 1
}

pub fn palindrome_permutation_kevin_2(string: &str) -> bool
{
    let mut frequencies = [0usize; 26];
    let string_bytes = string.as_bytes();
    for ch in string_bytes
    {
        if ch.is_ascii_alphabetic()
        {
            let ch_index = ch.to_ascii_lowercase() - b'a';
            frequencies[ch_index as usize] += 1;
        }
    }

    frequencies.iter().filter(|c| *c % 2 == 1).count() <= 1
}

pub fn palindrome_permutation_kevin_3(string: &str) -> bool
{
    let mut odd_even_bits = 0u32;
    let string_bytes = string.as_bytes();
    for ch in string_bytes
    {
        let ch_index = ch | 32;
        if b'a' <= ch_index && ch_index <= b'z'
        {
            odd_even_bits ^= 1 << (ch_index - b'a');
        }
    }

    odd_even_bits & odd_even_bits.wrapping_sub(1) == 0
}

pub fn palindrome_permutation_mike(string: &str) -> bool
{
    if string.len() <= 1
    {
        return true;
    }

    let letters: Vec<_> = string.trim().to_lowercase().chars().filter(|c| !c.is_whitespace() && c.is_alphabetic()).collect();
    let identical = letters.iter().all(|c| *c == letters[0]);

    if identical
    {
        return true;
    }

    let mut letter_freq: HashMap<char, u32> = HashMap::new();
    let mut is_palindrome = false;

    letters.iter().for_each(|l| {
                      *letter_freq.entry(*l).or_insert(0) += 1;
                  });

    let mut uneven_frequency = 0;
    letter_freq.into_iter().for_each(|(_key, val)| {
                               if val % 2 != 0
                               {
                                   uneven_frequency += 1
                               }
                           });

    if uneven_frequency <= 1
    {
        is_palindrome = true
    }

    is_palindrome
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_palindrome_permutation()
    {
        let test_cases = read_test_cases();
        for ref test_case in test_cases
        {
            let actual = palindrome_permutation(&test_case.input);
            assert_eq!(actual, test_case.answer, "input: '{}', expected: {}, actual: {}", test_case.input, test_case.answer, actual);
        }
    }
}
