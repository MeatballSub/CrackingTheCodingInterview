use crate::NamedStack;
use crate::SortableStack;

#[derive(Debug, Default)]
pub struct InsertionSortStack
{
    values: Vec<i32>,
}

impl InsertionSortStack
{
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl NamedStack for InsertionSortStack
{
    const NAME: &'static str = "insertion_sort_stack";
}

impl SortableStack for InsertionSortStack
{
    fn push(&mut self, value: i32) { self.values.push(value); }

    fn pop(&mut self) -> Option<i32> { self.values.pop() }

    fn peek(&mut self) -> Option<i32> { self.values.last().copied() }

    fn is_empty(&self) -> bool { self.values.is_empty() }

    fn len(&self) -> usize { self.values.len() }

    fn sort(&mut self)
    {
        let mut sorting_stack = Vec::with_capacity(self.values.len());
        while let Some(item) = self.pop()
        {
            while let Some(&sorting_top) = sorting_stack.last().filter(|&&held| held > item)
            {
                sorting_stack.pop();
                self.push(sorting_top);
            }
            sorting_stack.push(item);
        }
        while let Some(sorting_top) = sorting_stack.pop()
        {
            self.push(sorting_top);
        }
    }
}
