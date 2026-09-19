pub mod test;

// Queue via Stacks: Implement a MyQueue class which implements a queue using
// two stacks.
pub struct MyQueue {
    newest: Vec<i32>,
    oldest: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
         MyQueue{newest: Vec::new(), oldest: Vec::new()}
    }

    pub fn enqueue(&mut self, value: i32) {
        self.newest.push(value);

    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if !self.oldest.is_empty() {
            self.oldest.pop()
        } else {
            self.shift();
            self.oldest.pop()
        }
    }

    pub fn peek(&mut self) -> Option<i32> {
        if !self.oldest.is_empty() {
            self.oldest.last().copied()
        } else {
            self.shift();
            self.oldest.last().copied()
        }
    }

    fn shift (&mut self) {
        while let Some (newest_pop) = self.newest.pop() {
            self.oldest.push(newest_pop);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.newest.is_empty() && self.oldest.is_empty()
    }

    pub fn len(&self) -> usize {
        self.newest.len() + self.oldest.len()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_queue_via_stacks() {
        for case in read_test_cases() {
            let mut queue = MyQueue::new();
            run_operations(&mut queue, &case);
        }
    }
}
