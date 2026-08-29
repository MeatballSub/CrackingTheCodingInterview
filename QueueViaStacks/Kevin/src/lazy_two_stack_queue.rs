use crate::MyQueue;
use crate::NamedQueue;

#[derive(Debug, Default)]
pub struct LazyTwoStackQueue
{
    newest: Vec<i32>,
    oldest: Vec<i32>,
}

impl LazyTwoStackQueue
{
    #[must_use]
    pub fn new() -> Self { Self::default() }

    fn refill_oldest(&mut self)
    {
        if self.oldest.is_empty()
        {
            while let Some(value) = self.newest.pop()
            {
                self.oldest.push(value);
            }
        }
    }
}

impl NamedQueue for LazyTwoStackQueue
{
    const NAME: &'static str = "lazy_two_stack_queue";
}

impl MyQueue for LazyTwoStackQueue
{
    fn enqueue(&mut self, value: i32) { self.newest.push(value); }

    fn dequeue(&mut self) -> Option<i32>
    {
        self.refill_oldest();
        self.oldest.pop()
    }

    fn peek(&mut self) -> Option<i32>
    {
        self.refill_oldest();
        self.oldest.last().copied()
    }

    fn is_empty(&self) -> bool { self.newest.is_empty() && self.oldest.is_empty() }

    fn len(&self) -> usize { self.newest.len() + self.oldest.len() }
}
