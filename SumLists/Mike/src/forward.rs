use crate::linked_list::LinkedList;
fn front_pad(digits: Vec<u8>, target_len: usize) -> Vec<u8> {
    let pad = target_len - digits.len();
    let mut out = vec![0u8; pad];
    out.extend(digits);
    out
}

pub fn sum_lists_forward(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> {
    let mut list_a_vec = a.into_iter().collect::<Vec<u8>>();
    let mut list_b_vec = b.into_iter().collect::<Vec<u8>>();

    if list_a_vec.len() < list_b_vec.len() {
        list_a_vec = front_pad(list_a_vec, list_b_vec.len());
    } else if list_b_vec.len() < list_a_vec.len() {
        list_b_vec = front_pad(list_b_vec, list_a_vec.len());
    }
    list_a_vec.reverse();
    list_b_vec.reverse();
    let mut result = LinkedList::new();
    let mut carry = 0;
    
    for (&val_a, &val_b) in list_a_vec.iter().zip(list_b_vec.iter()) {
        let sum = val_a + val_b + carry;

        //what we keep for this column
        let remainder = sum % 10;
        carry = sum / 10;
        result.push_front(remainder);
    }

    if carry > 0 {
        result.push_front(carry);
    }

    result

}
