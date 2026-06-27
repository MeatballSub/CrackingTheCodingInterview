use crate::linked_list::LinkedList;
use crate::sum_utils::{int_to_linked_list, linked_list_to_int};

pub fn sum_lists_reverse(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> {
    let a_list_number = linked_list_to_int(a, true);
    let b_list_number = linked_list_to_int(b, true);
    let sum = a_list_number + b_list_number;
    int_to_linked_list(sum, true)
}
