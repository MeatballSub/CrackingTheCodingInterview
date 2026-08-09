//! Stack Min: How would you design a stack which, in addition to push and pop,
//! has a function min which returns the minimum element? Push, pop and min
//! should all operate in O(1) time.

pub mod auxiliary_min_stack;
pub mod paired_min_stack;
pub mod test;

pub trait MinStack
{
    fn push(&mut self, value: i32);

    fn pop(&mut self) -> Option<i32>;

    #[must_use]
    fn min(&self) -> Option<i32>;

    #[must_use]
    fn is_empty(&self) -> bool;
}
