use crate::linked_list::LinkedList;

pub fn linked_list_to_int(list: LinkedList<u8>, rev: bool) -> i32 {
    let mut list_vec = list.into_iter().collect::<Vec<u8>>();
    if rev {
        list_vec.reverse();
    }
    let string_num: String = list_vec.iter().map(|n| n.to_string()).collect();
    string_num.trim().parse().unwrap()
}

pub fn int_to_linked_list(num: i32, rev: bool) -> LinkedList<u8> {
    let mut num_vec: Vec<i32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    if rev {
        num_vec.reverse();
    }
    let mut list = LinkedList::new();
    for num in &num_vec {
        list.push_back((*num).try_into().unwrap());
    }

    list
}