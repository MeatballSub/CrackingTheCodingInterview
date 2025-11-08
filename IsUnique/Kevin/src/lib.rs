use std::collections::HashSet;

pub mod test;

pub fn is_unique_hash_set(string: &str) -> bool
{
    let mut seen = HashSet::new();
    for ch in string.chars()
    {
        if !seen.insert(ch)
        {
            return false;
        }
    }
    true
}

pub fn is_unique_no_data_structures(string: &str) -> bool
{
    let string_bytes = string.as_bytes();
    for i in 0..string_bytes.len()
    {
        for j in i + 1..string_bytes.len()
        {
            if string_bytes.get(i).unwrap() == string_bytes.get(j).unwrap()
            {
                return false;
            }
        }
    }
    true
}

pub fn is_unique_seven_bit_ascii(string: &str) -> bool
{
    let mut seen = [false; 128];
    let string_bytes = string.as_bytes();
    for ch in string_bytes
    {
        let flag = &mut seen[*ch as usize];
        if *flag
        {
            return false;
        }
        *flag = true;
    }
    true
}

pub fn is_unique(string: &str) -> bool
{
    // is_unique_hash_set(string)
    //is_unique_no_data_structures(string)
    is_unique_seven_bit_ascii(string)
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_is_unique()
    {
        let test_cases = read_test_cases();
        for ref test_case in test_cases
        {
            let actual = is_unique(&test_case.input);
            assert_eq!(actual, test_case.answer, "input: '{}', expected: {}, actual: {}", test_case.input, test_case.answer, actual);
        }
    }
}
