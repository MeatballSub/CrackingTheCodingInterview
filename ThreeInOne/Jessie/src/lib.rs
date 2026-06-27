pub mod test;

#[derive(Debug, PartialEq, Eq)]
pub enum StackError {
    Full,
    InvalidStack,
}

// Three in One: Describe how you could use a single array to implement three
// stacks.
pub struct FixedMultiStack {
    stack_capacity: usize,
    values: Vec<i32>,
    sizes: [usize; 3],
}

impl FixedMultiStack {
    pub fn new(stack_capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError> {
        todo!()
    }

    pub fn pop(&mut self, stack_num: usize) -> Option<i32> {
        todo!()
    }

    pub fn peek(&self, stack_num: usize) -> Option<i32> {
        todo!()
    }

    pub fn is_empty(&self, stack_num: usize) -> bool {
        todo!()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_three_in_one() {
        for case in read_test_cases() {
            let mut stack = FixedMultiStack::new(case.stack_capacity);
            run_operations(&mut stack, &case);
        }
    }
}
