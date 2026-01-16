pub mod test;

fn update_compressed_string(compressed_string: &mut String, character: char, num_chars: u32) {
    compressed_string.push_str(&character.to_string());
    compressed_string.push_str(&num_chars.to_string());
}

pub fn string_compression(string: &str) -> String {
    let mut compressed_string = String::new();

    let mut curr_char = string
        .chars()
        .nth(0)
        .expect("unable to get first char of input string");
    let mut num_chars: u32 = 0;

    for next_char in string.chars() {
        if !curr_char.eq(&next_char) {
            update_compressed_string(&mut compressed_string, curr_char, num_chars);

            curr_char = next_char;
            num_chars = 0;
        }

        num_chars += 1
    }
    update_compressed_string(&mut compressed_string, curr_char, num_chars);

    if compressed_string.len() < string.len() {
        compressed_string
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
