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
        MinStack{values: vec![], minimums: vec![]}
    }

    pub fn push(&mut self, value: i32) {
        self.values.push(value);
        if let Some(min) = self.min() {
            if value < min {
                self.minimums.push(value);
            } else {
                self.minimums.push(min);
            }

        } else {
            self.minimums.push(value);
        }

    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }
        self.minimums.pop();
        self.values.pop()

    }

    pub fn min(&self) -> Option<i32> { 
        self.minimums.iter().min().copied()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
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
