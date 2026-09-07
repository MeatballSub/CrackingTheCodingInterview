pub mod test;

// Sort Stack: Write a program to sort a stack such that the smallest items are
// on the top. You can use an additional temporary stack, but you may not copy
// the elements into any other data structure (such as an array). The stack
// supports the following operations: push, pop, peek, and isEmpty.
pub struct SortableStack {
    values: Vec<i32>,
}

impl SortableStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: i32) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn peek(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    // Reorders the stack so the smallest item is on top.
    pub fn sort(&mut self) {
        todo!()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_sort_stack() {
        for case in read_test_cases() {
            let mut stack = SortableStack::new();
            run_operations(&mut stack, &case);
        }
    }
}
