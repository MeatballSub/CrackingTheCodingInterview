pub mod test;
use std::collections::HashSet;

pub fn one_away(string1: &str, string2: &str) -> bool {
    let mut changes = 0;
    let abs_diff = (string1.chars().count() as isize - string2.chars().count() as isize).abs();

    if abs_diff > 1  {
        return false;
    } else if abs_diff == 1 {
        changes += 1;
    } else {
        let mismatches = string1
            .chars()
            .zip(string2.chars())
            .filter(|&(c1, c2)| {
                println!("{:?}", c1);
                println!("============");
                println!("{:?}", c2);
                c1 != c2
            })
            .count();
        return mismatches <= 1;
    }

    let string1_chars: HashSet<char> = string1.chars().collect();
    let string2_chars: HashSet<char> = string2.chars().collect();
    let mut unique_string_chars = Vec::new();

    string1_chars.iter().for_each(|c| {
        if !string2_chars.contains(c){
            unique_string_chars.push(*c);
            changes +=1;
        }
    });

    if abs_diff == 1 && unique_string_chars.len() <= 1 {
        changes = 1;
    }

    changes <= 1
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
