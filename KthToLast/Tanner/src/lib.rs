use crate::linked_list::LinkedList;

mod linked_list;
pub mod test;

// Implement an algorithm to find the k-th to last element of a singly linked
// list
pub fn kth_to_last(k: usize, list: &LinkedList<i32>) -> Option<&i32> {
    todo!()
}

// FOLLOW UP
// Implement an algorithm to find the k-th to last element of a singly linked
// list, without using the len of the list
pub fn kth_to_last_no_size(k: usize, list: &LinkedList<i32>) -> Option<&i32> {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_kth_to_last() {
        let test_cases = read_test_cases();
        for ref mut test_case in &test_cases {
            let actual = kth_to_last(test_case.k, &test_case.list);
            assert_eq!(
                actual.cloned(),
                test_case.answer,
                "Test case: {:?}\n    your answer: {:?}",
                test_case,
                actual
            );
        }
    }

    #[test]
    fn test_kth_to_last_no_size() {
        let test_cases = read_test_cases();
        for ref mut test_case in &test_cases {
            let actual = kth_to_last_no_size(test_case.k, &test_case.list);
            assert_eq!(
                actual.cloned(),
                test_case.answer,
                "Test case: {:?}\n    your answer: {:?}",
                test_case,
                actual
            );
        }
    }
}
