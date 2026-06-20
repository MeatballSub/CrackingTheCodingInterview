pub mod linked_list;
#[cfg(test)]
pub mod test;

mod forward;
mod reverse;

use crate::linked_list::LinkedList;

pub fn sum_lists_reverse(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> {
    reverse::sum_lists_reverse(a, b)
}

pub fn sum_lists_forward(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> {
    forward::sum_lists_forward(a, b)
}
