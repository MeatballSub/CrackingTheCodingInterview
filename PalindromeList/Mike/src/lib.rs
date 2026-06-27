use crate::linked_list::LinkedList;

mod linked_list;
pub mod test;

// Implement a function to check if a linked list is a palindrome.
pub fn is_palindrome(list: &LinkedList<i32>) -> bool {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_is_palindrome() {
        let test_cases = read_test_cases();
        for test_case in &test_cases {
            let actual = is_palindrome(&test_case.list);
            assert_eq!(
                actual, test_case.expected,
                "{}: input {:?}, expected: {}, actual: {}",
                test_case.description, test_case.list, test_case.expected, actual
            );
        }
    }
}
