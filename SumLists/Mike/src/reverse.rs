use crate::linked_list::LinkedList;

pub fn sum_lists_reverse(a: LinkedList<u8>, b: LinkedList<u8>) -> LinkedList<u8> {
    let mut list_a_vec = a.into_iter().collect::<Vec<u8>>();
    let mut list_b_vec = b.into_iter().collect::<Vec<u8>>();
    
    if list_a_vec.len() < list_b_vec.len() {
        list_a_vec.resize(list_b_vec.len(), 0);
    } else if list_b_vec.len() < list_a_vec.len() {
        list_b_vec.resize(list_a_vec.len(), 0);
    }
    
    list_a_vec.reverse();
    list_b_vec.reverse();

    let mut result = LinkedList::new();
    let mut carry = 0;

    for (&val_a, &val_b) in list_a_vec.iter().rev().zip(list_b_vec.iter().rev()) {
        let sum = val_a + val_b + carry;

        //what we keep for this column
        let remainder = sum % 10;
        carry = sum / 10;
        result.push_back(remainder);
    }
    
    if carry > 0 {
        result.push_back(carry);
    }
    
    result
}



