pub mod test;

// Stack Min: How would you design a stack which, in addition to push and pop,
// has a function min which returns the minimum element? Push, pop and min
// should all operate in O(1) time.
pub struct MinStack {
    values: Vec<i32>,
    minimums: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: i32) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn min(&self) -> Option<i32> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_stack_min() {
        for case in read_test_cases() {
            let mut stack = MinStack::new();
            run_operations(&mut stack, &case);
        }
    }
}
