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
impl<T: Clone + PartialOrd> LinkedList<T>
{
    pub fn partition_stable_safe(&mut self, partition_value: T)
    {
        let mut ge_list = LinkedList::new();

        let mut cursor = self.unsafe_cursor_front_mut();
        while let Some(current_val) = cursor.current()
        {
            if *current_val < partition_value
            {
                cursor.move_next();
            }
            else
            {
                let x = cursor.remove_current().unwrap();
                ge_list.push_back(x);
            }
        }
        self.append(&mut ge_list);
    }

    pub fn partition_unstable_unsafe(&mut self, partition_value: T)
    {
        if self.len() == 0
        {
            // Nothing to do
            return;
        }

        // Safe because there is at least one element in the list
        let mut tail = self.head;
        let mut next = self.head;

        while let Some(mut curr) = next
        {
            // safe because curr is some, and all nodes will have a next
            let curr_node = unsafe { curr.as_mut() };
            next = curr_node.next;
            if curr_node.element < partition_value
            {
                // assign to left partition
                curr_node.next = self.head;
                self.head = Some(curr);
            }
            else
            {
                // assign to right partition
                unsafe { tail.unwrap().as_mut() }.next = Some(curr);
                tail = Some(curr);
            }
        }

        unsafe { tail.unwrap().as_mut() }.next = None;
        self.tail = tail;
    }

    pub fn partition_stable_unsafe(&mut self, partition_value: T)
    {
        if self.len() == 0
        {
            // Nothing to do
            return;
        }

        // let mut lt_head: Option<std::ptr::NonNull<super::Node<T>>> = None;
        let mut lt_tail: Option<std::ptr::NonNull<super::Node<T>>> = None;
        let mut ge_head: Option<std::ptr::NonNull<super::Node<T>>> = None;
        let mut ge_tail: Option<std::ptr::NonNull<super::Node<T>>> = None;

        let mut next = self.head;
        while let Some(curr) = next
        {
            let curr_node = unsafe { curr.as_ref() };
            next = curr_node.next;
            if curr_node.element < partition_value
            {
                match lt_tail
                {
                    None => self.head = Some(curr),
                    Some(mut tail) => unsafe { tail.as_mut() }.next = Some(curr),
                }
                lt_tail = Some(curr);
            }
            else
            {
                match ge_tail
                {
                    None => ge_head = Some(curr),
                    Some(mut tail) => unsafe { tail.as_mut() }.next = Some(curr),
                }
                ge_tail = Some(curr);
            }
        }
        match lt_tail
        {
            None => self.head = ge_head,
            Some(mut tail) => unsafe { tail.as_mut() }.next = ge_head,
        }
        match ge_tail
        {
            None => ge_tail = lt_tail,
            Some(mut tail) => unsafe { tail.as_mut() }.next = None,
        }
        self.tail = ge_tail;
    }
}

#[cfg(test)]
pub mod unit_test
{
    use crate::test::read_test_cases;

    #[test]
    fn test_partition_stable_safe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_stable_safe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
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

    #[test]
    fn test_partition_stable_unsafe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_stable_unsafe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
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

    #[test]
    fn test_partition_unstable_unsafe()
    {
        let mut test_cases = read_test_cases();
        for ref mut test_case in &mut test_cases
        {
            println!("Test Case:");
            println!("Input: {:?} {{partition_value: {}}}", test_case.list, test_case.partition_value);
            test_case.list.partition_unstable_unsafe(test_case.partition_value);
            let mut ge_found = false;
            for item in &test_case.list
            {
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
