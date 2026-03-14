use crate::linked_list::LinkedList;

mod linked_list;
pub mod test;

// Write code to remove duplicates from an unsorted linked list.
pub fn remove_dups(list: &mut LinkedList<i32>) {
    todo!()
}

// FOLLOW UP
// How would you solve this problem if a tomporary buffer is not allowed
pub fn remove_dups_follow_up(list: &mut LinkedList<i32>) {
    todo!()
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;

    #[test]
    fn test_remove_dups() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let mut actual = test_case.list.clone();
            remove_dups(&mut actual);
            assert_eq!(
                actual, test_case.answer,
                "Test case: {:?}\n    your answer: {:?}",
                test_case, actual
            );
        }
    }

    #[test]
    fn test_remove_dups_follow_up() {
        let test_cases = read_test_cases();
        for ref mut test_case in test_cases {
            let mut actual = test_case.list.clone();
            remove_dups_follow_up(&mut actual);
            assert_eq!(
                actual, test_case.answer,
                "Test case: {:?}\n    your answer: {:?}",
                test_case, actual
            );
        }
    }
}
