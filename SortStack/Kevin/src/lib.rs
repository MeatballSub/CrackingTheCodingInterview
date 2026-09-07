//! Sort Stack: Write a program to sort a stack such that the smallest items are
//! on the top. You can use an additional temporary stack, but you may not copy
//! the elements into any other data structure (such as an array). The stack
//! supports the following operations: push, pop, peek, and isEmpty.

pub mod insertion_sort_stack;
pub mod test;

pub trait SortableStack
{
    fn push(&mut self, value: i32);

    fn pop(&mut self) -> Option<i32>;

    /// Takes `&mut self` so a lazily sorted implementation may settle first.
    #[must_use]
    fn peek(&mut self) -> Option<i32>;

    #[must_use]
    fn is_empty(&self) -> bool;

    #[must_use]
    fn len(&self) -> usize;

    /// Reorders the stack so the smallest item is on top.
    fn sort(&mut self);
}

pub trait NamedStack: SortableStack + Default
{
    const NAME: &'static str;
}
