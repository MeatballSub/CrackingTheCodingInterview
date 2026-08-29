//! Queue via Stacks: Implement a MyQueue class which implements a queue using
//! two stacks.

pub mod eager_two_stack_queue;
pub mod lazy_two_stack_queue;
pub mod test;

pub trait MyQueue
{
    fn enqueue(&mut self, value: i32);

    fn dequeue(&mut self) -> Option<i32>;

    #[must_use]
    fn peek(&mut self) -> Option<i32>;

    #[must_use]
    fn is_empty(&self) -> bool;

    #[must_use]
    fn len(&self) -> usize;
}

pub trait NamedQueue: MyQueue + Default
{
    const NAME: &'static str;
}
