// Recommended reading:
//     Notes.md in this lesson's folder (../Notes.md): the Rust book's
//     chapters 5, 6, 10.1-10.2 and Appendix C, with the lesson notes and FAQ
// Instructions:
//     A Registry<T> stores items and hands back an ItemId for each one. Make
//     every test pass, in three steps:
//       1. Write the trait impls marked "by hand" yourself; no derive.
//       2. The tests don't compile yet. Add derives to ItemId and
//          RegistryError until they do, but only the derives the tests (or
//          another derive) actually need. Above each derive list, write a
//          short comment naming what needs each one.
//       3. Fill in every todo!(). Add a trait bound only where the code needs
//          one, and put it on the impl block or method that needs it, never
//          on the struct.
//     Finish with all tests passing, no compiler warnings, and default cargo
//     clippy clean.

use std::fmt;

/// A handle to one item in a `Registry`: the index it was stored at.
pub struct ItemId(usize);

pub struct Registry<T> {
    items: Vec<T>,
}

/// Why `try_get` failed.
pub enum RegistryError {
    /// `id` is past the end of a registry holding `len` items.
    OutOfRange { id: ItemId, len: usize },
}

impl<T> Registry<T> {
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }

    /// Stores `item` and returns its id. Ids count up from 0.
    pub fn add(&mut self, item: T) -> ItemId {
        todo!()
    }

    #[must_use]
    pub fn get(&self, id: ItemId) -> Option<&T> {
        todo!()
    }

    /// Like `get`, but says why the lookup failed.
    pub fn try_get(&self, id: ItemId) -> Result<&T, RegistryError> {
        todo!()
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        todo!()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        todo!()
    }

    /// The largest item, or `None` if the registry is empty.
    #[must_use]
    pub fn largest(&self) -> Option<&T> {
        todo!()
    }
}

// By hand: Default for Registry<T>. Registry::<T>::default() must work for
// every T, including a T with no Default of its own.

// By hand: Display for Registry<T>. Prints the items in order, comma
// separated, inside square brackets: "[3, 1, 2]", or "[]" when empty.

// By hand: Display for RegistryError. Prints
// "id 7 is out of range (the registry holds 3)".

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    /// Has no derives at all, on purpose.
    struct Opaque;

    #[test]
    fn add_and_get() {
        let mut names = Registry::new();
        let a = names.add("Ferris");
        let b = names.add("Corro");
        assert_eq!(names.get(a), Some(&"Ferris"));
        assert_eq!(names.get(b), Some(&"Corro"));
        assert_eq!(names.len(), 2);
    }

    #[test]
    fn ids_are_values() {
        let mut names = Registry::new();
        let id = names.add("Ferris");
        let copy = id;
        assert_eq!(names.get(id), names.get(copy));
        assert_eq!(id, ItemId(0));
    }

    #[test]
    fn ids_in_sets_and_sorted() {
        let mut numbers = Registry::new();
        let mut ids = vec![numbers.add(10), numbers.add(20), numbers.add(30)];
        let unique: HashSet<ItemId> = ids.iter().copied().collect();
        assert_eq!(unique.len(), 3);
        ids.reverse();
        ids.sort();
        assert_eq!(ids, [ItemId(0), ItemId(1), ItemId(2)]);
    }

    #[test]
    fn missing_id() {
        let mut numbers = Registry::new();
        numbers.add(1);
        assert_eq!(numbers.get(ItemId(5)), None);
    }

    #[test]
    fn default_for_any_t() {
        let empty: Registry<Opaque> = Registry::default();
        assert!(empty.is_empty());
        let mut opaque = Registry::new();
        opaque.add(Opaque);
        assert_eq!(opaque.len(), 1);
    }

    #[test]
    fn largest() {
        let mut numbers = Registry::new();
        assert_eq!(numbers.largest(), None);
        numbers.add(3);
        numbers.add(9);
        numbers.add(4);
        assert_eq!(numbers.largest(), Some(&9));
    }

    #[test]
    fn display() {
        let mut numbers = Registry::new();
        assert_eq!(numbers.to_string(), "[]");
        numbers.add(3);
        numbers.add(1);
        numbers.add(2);
        assert_eq!(numbers.to_string(), "[3, 1, 2]");
    }

    #[test]
    fn try_get_error() {
        let mut names = Registry::new();
        let id = names.add("Ferris");
        assert_eq!(names.try_get(id), Ok(&"Ferris"));
        let err = names.try_get(ItemId(7)).unwrap_err();
        assert_eq!(
            err,
            RegistryError::OutOfRange {
                id: ItemId(7),
                len: 1
            }
        );
        assert_eq!(
            err.to_string(),
            "id 7 is out of range (the registry holds 1)"
        );
    }
}
