use crate::MinStack;

/// One stack entry: the value pushed, and the minimum of the whole stack at the
/// moment it was pushed.
#[derive(Clone, Copy, Debug)]
struct Entry
{
    value: i32,
    min: i32,
}

/// Every element carries its own running minimum, so `min()` is a peek at the
/// top entry. Costs one extra `i32` per element unconditionally, where
/// [`AuxiliaryMinStack`] stores minimums sparsely.
///
/// [`AuxiliaryMinStack`]: crate::auxiliary_min_stack::AuxiliaryMinStack
#[derive(Debug, Default)]
pub struct PairedMinStack
{
    values: Vec<Entry>,
}

impl PairedMinStack
{
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl MinStack for PairedMinStack
{
    fn push(&mut self, value: i32)
    {
        let min = self.min().map_or(value, |old_min| old_min.min(value));
        self.values.push(Entry { value, min });
    }

    fn pop(&mut self) -> Option<i32> { self.values.pop().map(|entry| entry.value) }

    fn min(&self) -> Option<i32> { self.values.last().map(|entry| entry.min) }

    fn is_empty(&self) -> bool { self.values.is_empty() }
}
