pub mod test;

pub fn one_away(string1: &str, string2: &str) -> bool
{
    if string1.len().abs_diff(string2.len()) > 1
    {
        return false;
    }

    let (short, long) = if string1.len() < string2.len()
    {
        (string1.as_bytes(), string2.as_bytes())
    }
    else
    {
        (string2.as_bytes(), string1.as_bytes())
    };

    let short_len = short.len();
    let long_len = long.len();
    let same_size = short_len == long_len;

    let mut i = 0;
    let mut j = 0;
    let mut found = false;
    while i < short_len && j < long_len
    {
        if short[i] != long[j]
        {
            if found
            {
                return false;
            }
            found = true;
            if same_size
            {
                i += 1;
            }
        }
        else
        {
            i += 1;
        }
        j += 1;
    }
    true
}

#[cfg(test)]
pub mod unit_test
{
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_one_away()
    {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases
        {
            let actual = one_away(&test_case.string1, &test_case.string2);
            assert_eq!(actual, test_case.answer,
                       "input: '{},{}', expected: {}, actual: {}",
                       test_case.string1, test_case.string2, test_case.answer, actual);
        }
    }
}
