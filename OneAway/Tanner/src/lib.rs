pub mod test;

pub fn one_away(string1: &str, string2: &str) -> bool { todo!("Implement this function") }

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
