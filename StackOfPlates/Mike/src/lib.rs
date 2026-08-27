pub mod test;

// Stack of Plates: Imagine a (literal) stack of plates. If the stack gets too
// high, it might topple. Therefore, in real life, we would likely start a new
// stack when the previous stack exceeds some threshold. Implement a data
// structure SetOfStacks that mimics this. SetOfStacks should be composed of
// several stacks and should create a new stack once the previous one exceeds
// capacity. SetOfStacks.push() and SetOfStacks.pop() should behave identically
// to a single stack (that is, pop() should return the same values as it would
// if there were just a single stack).
//
// FOLLOW UP
//
// Implement a function popAt(int index) which performs a pop operation on a
// specific sub-stack.
pub struct SetOfStacks {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
}

impl SetOfStacks {
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&mut self, value: i32) {
        todo!()
    }

    pub fn pop(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn pop_at(&mut self, index: usize) -> Option<i32> {
        todo!()
    }

    pub fn peek(&self) -> Option<i32> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn stack_count(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_stack_of_plates() {
        for case in read_test_cases() {
            let mut stack = SetOfStacks::new(case.capacity);
            run_operations(&mut stack, &case);
        }
    }
}
