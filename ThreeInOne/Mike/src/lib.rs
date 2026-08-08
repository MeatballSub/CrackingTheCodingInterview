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
        let placeholder = 0;
        Self {
            stack_capacity,
            values:vec![placeholder; stack_capacity * 3],
            sizes: [0; 3],
        }
    }

    pub fn push(&mut self, stack_num: usize, value: i32) -> Result<(), StackError> {
        if self.is_full(stack_num) {
            return Err(StackError::Full);
        }
        let next_free = self.next_free_slot(stack_num);
        self.values[next_free] = value;
        self.sizes[stack_num] += 1;
        Ok(())
    }

    pub fn pop(&mut self, stack_num: usize) -> Option<i32> {
        if self.is_empty(stack_num) {
            return None;
        }
        let target_val = self.next_free_slot(stack_num) - 1;
        self.sizes[stack_num] -= 1;
        Some(self.values[target_val])
    }

    pub fn peek(&self, stack_num: usize) -> Option<i32> {
        if self.is_empty(stack_num) {
            return None;
        }
        let target_val = self.next_free_slot(stack_num) - 1;
        Some(self.values[target_val])
    }

    pub fn is_empty(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] == 0
    }

    pub fn is_full(&self, stack_num: usize) -> bool {
        self.sizes[stack_num] >= self.stack_capacity
    }

    fn next_free_slot(&self, stack_number: usize) -> usize {
        stack_number * self.stack_capacity + self.sizes[stack_number]
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
