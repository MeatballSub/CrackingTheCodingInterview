use std::collections::HashMap;

pub mod test;

pub fn palindrome_permutation(string: &str) -> bool {
    if string.len() <= 1  {
        println!("===============");
        println!("short circuit evaluation");
        println!("{:?}", string);
        println!("===============");
        return true
    }

    let letters: Vec<_> = string.trim().to_lowercase().chars().filter(|c| !c.is_whitespace() && c.is_alphabetic()).collect();
    let identical = letters.iter().all(|c| *c == letters[0]);

    if identical {
        return true;
    }

    let mut letter_freq: HashMap<char, u32> = HashMap::new();
    let mut is_palindrome = false;


    letters.iter().for_each(|l| {
        *letter_freq.entry(*l).or_insert(0) += 1;
    });

    let mut occurrence_of_one = 0;
    let mut otherwise_symmetrical = false;

    letter_freq.into_iter().for_each(|(key, val)| {
        if val == 1{
            occurrence_of_one +=1
        } if val % 2 == 0 {
            otherwise_symmetrical = true
        } else {
            otherwise_symmetrical = false
        }
    });
   
   
    if occurrence_of_one == 1 && otherwise_symmetrical == true {
        is_palindrome = true
    }
    println!("===============");
    println!("{:?}", string);
    println!("{:?}", occurrence_of_one);
    println!("{:?}", otherwise_symmetrical);
    println!("{:?}", is_palindrome);
    println!("===============");
    is_palindrome
    
}


#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_palindrome_permutation() {
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
