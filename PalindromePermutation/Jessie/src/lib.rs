pub mod test;

// Given a string, write a function to check if it is a permutation of a
// palindrome.  A palindrome is a word or phrase that is the same forwards and
// backwards.  A permutation is a rearrangement of letters.  The palindrome
// does not need to be limited to just dictionary words.  You can ignore casing
// and non-letter characters.
// EXAMPLE:
// Input:  Tact Coa
// Output: True (permutations: "taco cat", "atco cta", etc.)
pub fn palindrome_permutation(string: &str) -> bool {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_urlify() {
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
