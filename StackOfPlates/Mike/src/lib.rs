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
        Self {
            capacity,
            stacks: Vec::new(),
        }
    }

    pub fn push(&mut self, value: i32) {
        if self.is_empty() || self.top_stack_count() == self.capacity {
            self.stacks.push(vec![value]);
        } else if self.top_stack_count() < self.capacity {
            if let Some(inner_vec) = self.stacks.last_mut() {
                inner_vec.push(value);
            }
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let result = self.stacks.last_mut().unwrap().pop();
            if self.top_stack_count() == 0 {
                self.stacks.pop();
            }

            result
        }
    }

    pub fn pop_at(&mut self, index: usize) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let safe_indexed_target = self.stacks.get_mut(index)?;
            let result = safe_indexed_target.pop();
    
            self.recombobulate();
            
            result
        }
    }

    pub fn peek(&self) -> Option<i32> {
        if self.top_stack_count() == 0 || self.is_empty() {
            None
        } else {
            self.stacks.last().and_then(|stack| stack.last().copied())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stacks.iter().all(|v| v.is_empty())
    }

    pub fn top_stack_count(&self) -> usize {
        self.stacks.last().map(|v| v.len()).unwrap_or(0)
    }

    pub fn stack_count(&self) -> usize {
        self.stacks.len()
    }

    pub fn recombobulate(&mut self) -> () {
        let flattened_stacks: Vec<_> = self.stacks.clone().into_iter().flatten().collect();
        let recombobulated = flattened_stacks
            .chunks(self.capacity)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        self.stacks = recombobulated;
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
