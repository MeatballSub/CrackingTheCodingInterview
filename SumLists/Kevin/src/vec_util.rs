use crate::linked_list::LinkedList;

pub fn sum_lists_reverse_vec(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8>
{
    let mut result = LinkedList::new();
    let index_fn = |i, _| i;
    let builder = |val| result.push_back(val);
    sum_lists_vec(a, b, index_fn, builder);
    result
}

pub fn sum_lists_forward_vec(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8>
{
    let mut result = LinkedList::new();
    let index_fn = |i, slice_len: usize| slice_len.wrapping_sub(i + 1);
    let builder = |val| result.push_front(val);
    sum_lists_vec(a, b, index_fn, builder);
    result
}

fn sum_lists_vec(a: LinkedList<u8>, b: LinkedList<u8>, index_fn: impl Fn(usize, usize) -> usize, mut builder: impl FnMut(u8))
{
    let (a, b, len) = to_vecs(a, b);
    let mut carry = 0u8;

    for i in 0..len
    {
        let a_idx = index_fn(i, a.len());
        let b_idx = index_fn(i, b.len());
        let sum = a.get(a_idx).unwrap_or(&0) + b.get(b_idx).unwrap_or(&0) + carry;
        carry = sum / 10;
        builder(sum % 10);
    }

    if carry > 0
    {
        builder(carry);
    }
}

fn to_vecs(a: LinkedList<u8>, b: LinkedList<u8>) -> (Vec<u8>, Vec<u8>, usize)
{
    let a_len = a.len();
    let b_len = b.len();
    let mut a_vec = Vec::with_capacity(a_len);
    let mut b_vec = Vec::with_capacity(b_len);
    a_vec.extend(a);
    b_vec.extend(b);
    (a_vec, b_vec, a_len.max(b_len))
}
