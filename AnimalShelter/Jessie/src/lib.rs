pub mod test;

use std::collections::VecDeque;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum Species {
    Dog,
    Cat,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Animal {
    pub species: Species,
    pub id: u32,
}

// Animal Shelter: An animal shelter, which holds only dogs and cats, operates
// on a strictly "first in, first out" basis. People must adopt either the
// "oldest" (based on arrival time) of all animals at the shelter, or they can
// select whether they would prefer a dog or a cat (and will receive the oldest
// animal of that type). They cannot select which specific animal they would
// like. Create the data structures to maintain this system and implement
// operations such as enqueue, dequeueAny, dequeueDog, and dequeueCat. You may
// use the built-in LinkedList data structure.
pub struct AnimalShelter {
    dogs: VecDeque<Animal>,
    cats: VecDeque<Animal>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        todo!()
    }

    pub fn enqueue(&mut self, animal: Animal) {
        todo!()
    }

    // Adopts the oldest animal of either species.
    pub fn dequeue_any(&mut self) -> Option<Animal> {
        todo!()
    }

    // Adopts the oldest dog.
    pub fn dequeue_dog(&mut self) -> Option<Animal> {
        todo!()
    }

    // Adopts the oldest cat.
    pub fn dequeue_cat(&mut self) -> Option<Animal> {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
pub mod unit_test {
    use super::*;
    use crate::test::read_test_cases;
    use crate::test::run_operations;

    #[test]
    fn test_animal_shelter() {
        for case in read_test_cases() {
            let mut shelter = AnimalShelter::new();
            run_operations(&mut shelter, &case);
        }
    }
}
