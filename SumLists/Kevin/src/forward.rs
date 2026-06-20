use crate::linked_list::LinkedList;

pub fn sum_lists_forward_iterative(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8>
{
    let mut a = a;
    let mut b = b;
    pad_lists(&mut a, &mut b);

    let mut digit_stack: Vec<u8> = a.into_iter().zip(b).map(|(a, b)| a + b).collect();

    let mut result = LinkedList::new();
    let mut carry = 0u8;
    while let Some(raw) = digit_stack.pop()
    {
        let sum = raw + carry;
        carry = sum / 10;
        result.push_front(sum % 10);
    }

    if carry > 0
    {
        result.push_front(carry);
    }

    result
}

pub fn sum_lists_forward_recursive(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8>
{
    let mut a = a;
    let mut b = b;
    pad_lists(&mut a, &mut b);

    let (carry, mut result) = add_recursive(&mut a.into_iter(), &mut b.into_iter());
    if carry > 0
    {
        result.push_front(carry);
    }

    result
}

fn add_recursive(a: &mut impl Iterator<Item = u8>, b: &mut impl Iterator<Item = u8>) -> (u8, LinkedList<u8>)
{
    match (a.next(), b.next())
    {
        (None, None) => (0, LinkedList::new()),
        (a_digit, b_digit) =>
        {
            let (carry, mut result) = add_recursive(a, b);
            let sum = a_digit.unwrap_or(0) + b_digit.unwrap_or(0) + carry;
            result.push_front(sum % 10);
            (sum / 10, result)
        }
    }
}

fn pad_lists(a: &mut LinkedList<u8>, b: &mut LinkedList<u8>)
{
    let a_len = a.len();
    let b_len = b.len();

    if a_len < b_len
    {
        (0..b_len - a_len).for_each(|_| a.push_front(0));
    }
    else
    {
        (0..a_len - b_len).for_each(|_| b.push_front(0));
    }
}
