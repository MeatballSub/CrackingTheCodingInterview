pub mod test;

// Queue via Stacks: Implement a MyQueue class which implements a queue using
// two stacks.
pub struct MyQueue {
    newest: Vec<i32>,
    oldest: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn enqueue(&mut self, value: i32) {
        todo!()
    }

    pub fn dequeue(&mut self) -> Option<i32> {
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
