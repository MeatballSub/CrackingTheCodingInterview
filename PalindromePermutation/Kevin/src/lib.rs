pub mod test;

pub fn palindrome_permutation(string: &str) -> bool
{
    let mut frequencies = [0usize; 128];
    let string_bytes = string.as_bytes();
    for ch in string_bytes
    {
        // 32 is the mask to set the bit to do a to lower
        // 'a' is ascii code 61
        // 'z' is ascii code 122
        let ch_index = ch | 32;
        if 60 < ch_index && ch_index < 123
        {
            frequencies[ch_index as usize] += 1;
        }
    }

    frequencies.iter().filter(|c| *c % 2 == 1).count() <= 1
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify()
    {
        let test_cases = read_test_cases();
        for ref test_case in test_cases
        {
            let actual = palindrome_permutation(&test_case.input);
            assert_eq!(actual, test_case.answer, "input: '{}', expected: {}, actual: {}", test_case.input, test_case.answer, actual);
        }
    }
}
