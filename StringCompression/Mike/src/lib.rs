use std::collections::HashMap;

pub mod test;

pub fn string_compression(string: &str) -> String {
    if string.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut chars = string.chars().peekable();
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    while let Some(&next_char) = chars.peek() {
        chars.next();
        if next_char == current_char {
            count += 1;
        } else {
            result.push(current_char);
            result.push_str(&count.to_string());
            current_char = next_char;
            count = 1;
        }
    }

    result.push(current_char);
    result.push_str(&count.to_string());

    if result.len() < string.len() {
        result
    } else {
        string.to_string()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_string_compression() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let actual = string_compression(&test_case.string);
            assert_eq!(
                actual, test_case.answer,
                "input: '{}', expected: {}, actual: {}",
                test_case.string, test_case.answer, actual
            );
        }
    }
}
