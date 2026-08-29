use crate::MyQueue;
use crate::NamedQueue;

fn drain(source: &mut Vec<i32>, target: &mut Vec<i32>)
{
    while let Some(value) = source.pop()
    {
        target.push(value);
    }
}

#[derive(Debug, Default)]
pub struct EagerTwoStackQueue
{
    newest: Vec<i32>,
    oldest: Vec<i32>,
}

impl EagerTwoStackQueue
{
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl NamedQueue for EagerTwoStackQueue
{
    const NAME: &'static str = "eager_two_stack_queue";
}

impl MyQueue for EagerTwoStackQueue
{
    fn enqueue(&mut self, value: i32)
    {
        drain(&mut self.oldest, &mut self.newest);
        self.newest.push(value);
        drain(&mut self.newest, &mut self.oldest);
    }

    fn dequeue(&mut self) -> Option<i32> { self.oldest.pop() }

    fn peek(&mut self) -> Option<i32> { self.oldest.last().copied() }

    fn is_empty(&self) -> bool { self.newest.is_empty() && self.oldest.is_empty() }

    fn len(&self) -> usize { self.newest.len() + self.oldest.len() }
}
