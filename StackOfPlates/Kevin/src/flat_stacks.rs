use crate::SetOfStacks;

#[derive(Debug)]
pub struct FlatStacks
{
    capacity: usize,
    plates: Vec<i32>,
}

impl FlatStacks
{
    #[must_use]
    pub fn new(capacity: usize) -> Self { Self { capacity, plates: Vec::new() } }
}

impl SetOfStacks for FlatStacks
{
    fn push(&mut self, value: i32) { self.plates.push(value); }

    fn pop(&mut self) -> Option<i32> { self.plates.pop() }

    fn pop_at(&mut self, index: usize) -> Option<i32>
    {
        if index >= self.stack_count()
        {
            return None;
        }

        let top_of_sub_stack = ((index + 1) * self.capacity).min(self.plates.len()) - 1;
        Some(self.plates.remove(top_of_sub_stack))
    }

    fn peek(&self) -> Option<i32> { self.plates.last().copied() }

    fn is_empty(&self) -> bool { self.plates.is_empty() }

    fn stack_count(&self) -> usize { self.plates.len().div_ceil(self.capacity) }
}
