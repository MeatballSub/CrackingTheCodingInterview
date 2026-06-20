#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod linked_list;
#[cfg(test)]
pub mod test;

mod forward;
mod reverse;
mod vec_util;

use crate::linked_list::LinkedList;

pub fn sum_lists_reverse(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> { reverse::sum_lists_reverse(a, b) }

pub fn sum_lists_forward_iterative(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> { forward::sum_lists_forward_iterative(a, b) }

pub fn sum_lists_forward_recursive(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> { forward::sum_lists_forward_recursive(a, b) }

pub fn sum_lists_reverse_vec(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> { vec_util::sum_lists_reverse_vec(a, b) }

pub fn sum_lists_forward_vec(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> { vec_util::sum_lists_forward_vec(a, b) }
