use std::fmt::Write;

pub mod test;

pub fn string_compression(string: &str) -> String {
    let mut compressed_str = String::new();
    let mut consecutive = 0;
    let byte_string = string.as_bytes();

    for (i, c) in byte_string.iter().enumerate()
    {
        consecutive += 1;

        if i + 1 >= byte_string.len() || byte_string[i] != byte_string[i + 1]
        {
            let _ = write!(&mut compressed_str, "{}", *c as char);
            let _ = write!(&mut compressed_str, "{}", consecutive);
            consecutive = 0;
        }
    }

    if compressed_str.len() < byte_string.len() { compressed_str } else { string.to_string() }
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
