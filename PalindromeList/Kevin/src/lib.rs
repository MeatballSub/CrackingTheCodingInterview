#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::collections::VecDeque;

use crate::linked_list::LinkedList;

pub mod linked_list;
pub mod test;

pub fn is_palindrome_reverse_and_compare(list: &LinkedList<i32>) -> bool
{
    let mut rev_list = LinkedList::new();
    for element in list
    {
        rev_list.push_front(element);
    }

    list.iter().eq(rev_list)
}

pub fn is_palindrome_to_vec_reverse_and_compare(list: &LinkedList<i32>) -> bool
{
    let mut vec: Vec<&i32> = Vec::with_capacity(list.len());
    vec.extend(list.iter());
    list.iter().eq(vec.into_iter().rev())
}

pub fn is_palindrome_deque(list: &LinkedList<i32>) -> bool
{
    let mut deque: VecDeque<&i32> = VecDeque::with_capacity(list.len());
    deque.extend(list.iter());

    while let (Some(front), Some(back)) = (deque.pop_front(), deque.pop_back())
    {
        if front != back
        {
            return false;
        }
    }
    true
}

pub fn is_palindrome_two_pointer(list: &LinkedList<i32>) -> bool
{
    let mut vec: Vec<&i32> = Vec::with_capacity(list.len());
    vec.extend(list.iter());

    let mut lo = 0;
    let mut hi = vec.len();
    while lo < hi
    {
        hi -= 1;
        if vec[lo] != vec[hi]
        {
            return false;
        }
        lo += 1;
    }
    true
}

pub fn is_palindrome_stack_half(list: &LinkedList<i32>) -> bool
{
    let len = list.len();
    let half_len = len / 2;
    let mut iter = list.iter();

    let mut first_half: Vec<&i32> = Vec::with_capacity(half_len);
    first_half.extend(iter.by_ref().take(half_len));

    if len % 2 == 1
    {
        iter.next();
    }

    iter.eq(first_half.into_iter().rev())
}

pub fn is_palindrome_reverse_half(list: &LinkedList<i32>) -> bool
{
    let len = list.len();
    let half_len = len / 2;
    let mut iter = list.iter();

    let mut stack = LinkedList::new();
    iter.by_ref().take(half_len).for_each(|element| stack.push_front(element));

    if len % 2 == 1
    {
        iter.next();
    }

    iter.eq(stack)
}
