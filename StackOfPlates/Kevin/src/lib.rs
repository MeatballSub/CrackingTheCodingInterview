//! Stack of Plates: Imagine a (literal) stack of plates. If the stack gets too
//! high, it might topple. Therefore, in real life, we would likely start a new
//! stack when the previous stack exceeds some threshold. Implement a data
//! structure SetOfStacks that mimics this. SetOfStacks should be composed of
//! several stacks and should create a new stack once the previous one exceeds
//! capacity. SetOfStacks.push() and SetOfStacks.pop() should behave identically
//! to a single stack (that is, pop() should return the same values as it would
//! if there were just a single stack).
//!
//! FOLLOW UP
//!
//! Implement a function popAt(int index) which performs a pop operation on a
//! specific sub-stack.

pub mod flat_stacks;
pub mod no_rollover_stacks;
pub mod rollover_stacks;
pub mod test;

pub trait SetOfStacks
{
    fn push(&mut self, value: i32);

    fn pop(&mut self) -> Option<i32>;

    fn pop_at(&mut self, index: usize) -> Option<i32>;

    #[must_use]
    fn peek(&self) -> Option<i32>;

    #[must_use]
    fn is_empty(&self) -> bool;

    #[must_use]
    fn stack_count(&self) -> usize;
}
