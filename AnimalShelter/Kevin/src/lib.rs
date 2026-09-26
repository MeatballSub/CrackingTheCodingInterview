//! Animal Shelter: An animal shelter, which holds only dogs and cats, operates
//! on a strictly "first in, first out" basis. People must adopt either the
//! "oldest" (based on arrival time) of all animals at the shelter, or they can
//! select whether they would prefer a dog or a cat (and will receive the oldest
//! animal of that type). They cannot select which specific animal they would
//! like. Create the data structures to maintain this system and implement
//! operations such as enqueue, dequeueAny, dequeueDog, and dequeueCat. You may
//! use the built-in LinkedList data structure.

use serde::Deserialize;

pub mod test;
pub mod two_queue_shelter;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Species
{
    Dog,
    Cat,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Animal
{
    pub species: Species,
    pub id: u32,
}

pub trait AnimalShelter
{
    fn enqueue(&mut self, animal: Animal);

    /// Adopts the oldest animal of either species.
    fn dequeue_any(&mut self) -> Option<Animal>;

    /// Adopts the oldest dog.
    fn dequeue_dog(&mut self) -> Option<Animal>;

    /// Adopts the oldest cat.
    fn dequeue_cat(&mut self) -> Option<Animal>;

    #[must_use]
    fn is_empty(&self) -> bool;

    #[must_use]
    fn len(&self) -> usize;
}

pub trait NamedShelter: AnimalShelter + Default
{
    const NAME: &'static str;
}
