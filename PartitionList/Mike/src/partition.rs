use crate::linked_list::LinkedList;

// Write code to partition a linked list around  value(partition_value), such
// that all nodes less than partition_value come before all nodes greater than
// or equal to partition_value. (IMPORTANT: The partition element can appear
// anywhere in the right side partition; it does not need to appear between the
// left and right partitions.  The additional spacing in the example below
// indicates the partition.  Yes, the output below is one of many valid
// outputs!) EXAMPLE:
//  Input: 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1 [partition_value = 5]
// Output: 3 -> 2 -> 2     ->     10 -> 5 -> 5 -> 8

// NOTE: We're implementing a new method on the LinkedList itself
//       We have access to private data through self


impl<T: Clone + PartialOrd + std::fmt::Debug + std::cmp::Ord> LinkedList<T> {
    pub fn partition(&mut self, partition_value: T) {
        let mut values = self.iter().cloned().collect::<Vec<_>>();
        values.sort_by_key(|a| (a >= &partition_value, a.clone()));

        while self.pop_front().is_some() {}        
        
        for item in values {
            self.push_back(item);
        }
    }
}

#[cfg(test)]
pub mod unit_test {
    use crate::test::read_test_cases;

    #[test]
    fn test_partition() {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases {
            println!("Test Case:");
            println!(
                "Input: {:?} {{partition_value: {}}}",
                test_case.list, test_case.partition_value
            );
            test_case.list.partition(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list {
                let is_ge = *item >= test_case.partition_value;
                ge_found |= is_ge;
                let symbol = if is_ge { ">=" } else { "<" };
                println!("{} {} {}", item, symbol, test_case.partition_value);
                assert!(is_ge == ge_found,
                        "item less than the partition value found after values greater than or equal to the partition value");
            }
            println!("Ok");
            println!("Output: {:?}", test_case.list);
            println!();
        }
    }
}
