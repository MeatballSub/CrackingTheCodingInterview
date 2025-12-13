pub mod test;
use std::collections::{HashSet, HashMap};

pub fn one_away(string1: &str, string2: &str) -> bool {
    let abs_diff = (string1.chars().count() as isize - string2.chars().count() as isize).abs();

    if abs_diff > 1  {
        return false;
    } else if abs_diff == 1 {
        let largest_string = if string1.len() > string2.len() { string1} else { string2 };
        let smallest_string = if string1.len() < string2.len() { string1} else { string2 };

        let mut combinations = largest_string.chars().enumerate().map(|(i, c)| {
            let mut new_string = largest_string.clone().to_string();
            new_string.remove(i);
            new_string
        });

        return combinations.any(|entry | entry == smallest_string.to_string());

    } else {

        // try to do this but make it work for all examples.Basically incorporate above logic. use offset of length on larger string to determine how many changes away
        let mismatches = string1
            .chars()
            .zip(string2.chars())
            .filter(|&(c1, c2)| c1 != c2)
            .count();
        return mismatches <= 1;
    }
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
