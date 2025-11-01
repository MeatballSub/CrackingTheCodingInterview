pub mod test;

//faster, more memory intensive
pub fn is_unique(string: &str) -> bool {

    if string.chars().count() > 128  {
        return false
    }

    const limit: usize = 128;
    let mut charHash = [false; limit];

    for c in string.chars() {
        let charCode = u32::from(c);

        if charHash[charCode as usize] == true {
            return false
        }

        charHash[charCode as usize] = true;
        println!("Character: {}", c);
    }

    return true
}

// order of magnitude slower, less memory utilization
pub fn is_unique_no_data_structures(string: &str) -> bool {
    let mut unique = true;
    for (i, c) in string.chars().enumerate() {
        for sub_c in string.chars().skip(i + 1) {
            if c == sub_c {
                unique = false;
                break
            }
        }
    }

    unique
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_is_unique() {
        let test_cases = read_test_cases();
        for ref test_case in test_cases {
            let actual = is_unique(&test_case.input);
            assert_eq!(
                actual, test_case.answer,
                "input: '{}', expected: {}, actual: {}",
                test_case.input, test_case.answer, actual
            );
        }
    }
}
