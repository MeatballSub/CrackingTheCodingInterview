use crate::linked_list::LinkedList;

pub fn sum_lists_reverse(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8>
{
    let mut result = LinkedList::new();
    let mut carry = 0u8;
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();

    loop
    {
        let a_digit = a_iter.next();
        let b_digit = b_iter.next();
        if a_digit.is_none() && b_digit.is_none() && carry == 0
        {
            break;
        }
        let sum = a_digit.unwrap_or(0) + b_digit.unwrap_or(0) + carry;
        carry = sum / 10;
        result.push_back(sum % 10);
    }

    result
}
