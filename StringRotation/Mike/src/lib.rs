pub mod test;

// Assume you have a method is_substring which checks if one word is a substring
// of another. Given two strings, s1 and s2, write code to check if s2 is a
// rotation of s1 using only one call to is_substring
// example: "waterbottle" is a rotation of "erbottlewat"
pub fn string_rotation<F>(s1: &str, s2: &str, mut is_substring: F) -> bool
where
    F: FnMut(&str, &str) -> bool,
{
    // example call of is_substring, passing 2 strings
    // it returns true if string2 is a substring of string1
    // is_substring(string1, string2);
    // todo!();

    if s1.is_empty() && s2.is_empty() {
        return true;
    }

    if s1.len() != s2.len() {
        return false;
    }

    let s1_copy = s1.clone();
    let s1_combined = format!("{}{}", s1, s1_copy);
    return is_substring(&s1_combined, s2);


    // let mut s1_chars: Vec<char> = s1.chars().collect();
    // let s2_chars: Vec<char> = s2.chars().collect();
    // let n = s1.len();
    // let mut i = 0;

    // while i < n {
    //     i+= 1;

    //     if s1_chars.iter().eq(s2_chars.iter()) {
    //         return true;
    //     }

    //     let last = s1_chars.pop();
    //     s1_chars.insert(0, last.unwrap());
    // }

    // return false
}

pub struct LimitedSubstring(bool);

impl LimitedSubstring {
    pub fn new() -> Self {
        Self(false)
    }

    pub fn is_substring(&mut self, s1: &str, s2: &str) -> bool {
        if self.0 {
            panic!("is_substring already called once!");
        }

        self.0 = true;

        s1.contains(s2)
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_string_rotation() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let mut is_substring_provider = LimitedSubstring::new();
            let closure = |s1: &str, s2: &str| is_substring_provider.is_substring(s1, s2);
            let actual = string_rotation(&test_case.s1, &test_case.s2, closure);
            assert_eq!(
                actual, test_case.answer,
                "Test case: {:?}\n    your answer: {}",
                test_case, actual
            );
        }
    }
}
