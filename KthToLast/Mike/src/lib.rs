use std::ptr::NonNull;

use crate::linked_list::{LinkedList, Node};

mod linked_list;
pub mod test;

// Implement an algorithm to find the k-th to last element of a singly linked
// list
pub fn kth_to_last(k: usize, list: &LinkedList<i32>) -> Option<&i32> {
    let index: usize = list.len().checked_sub(1)?.checked_sub(k)?;
    list.iter().nth(index)
}

// FOLLOW UP
// Implement an algorithm to find the k-th to last element of a singly linked
// list, without using the len of the list
pub fn kth_to_last_no_size(k: usize, list: &LinkedList<i32>) -> Option<&i32> {
    let mut leader = list.head_node();
    let mut trailer = list.head_node();

    for _ in 0..=k {
        leader = match leader {
            Some(node) => unsafe { node.as_ref().next},
            None => return None,
        };
    }
    while leader.is_some() {
        leader = leader.and_then(|n| unsafe { n.as_ref().next });
        trailer = trailer.and_then(|n| unsafe { n.as_ref().next });
    }

    trailer.map(|node| unsafe { &node.as_ref().element })
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
