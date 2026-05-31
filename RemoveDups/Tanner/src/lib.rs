use std::collections::HashSet;

use crate::linked_list::LinkedList;

mod linked_list;
pub mod test;

// Write code to remove duplicates from an unsorted linked list.
pub fn remove_dups(list: &mut LinkedList<i32>) {
    let mut cursor = list.unsafe_cursor_front_mut();
    let mut entries = HashSet::<i32>::new();

    while let Some(current) = cursor.current() {
        if entries.insert(*current) {
            cursor.move_next();
        } else {
            cursor.remove_current();
        }
    }
}

// FOLLOW UP
// How would you solve this problem if a temporary buffer is not allowed
pub fn remove_dups_follow_up(list: &mut LinkedList<i32>) {
    let mut cursor = list.unsafe_cursor_front_mut();
    let mut next_cursor = list.unsafe_cursor_front_mut();

    while let Some(current) = cursor.current() {
        next_cursor.move_next();
        while let Some(next) = next_cursor.current() {
            if current == next {
                next_cursor.remove_current();
            } else {
                next_cursor.move_next();
            }
        }

        cursor.move_next();
        next_cursor = cursor.clone();
    }
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
