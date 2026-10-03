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
    arrivals: VecDeque<Animal>,
}

impl AnimalShelter {
    pub fn new() -> Self {
        AnimalShelter{
            dogs: VecDeque::new(),
            cats: VecDeque::new(),
            arrivals: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, animal: Animal) {
        self.arrivals.push_back(animal);
        match animal.species {
            Species::Dog => {
                self.dogs.push_back(animal);
            },
            Species::Cat => {
                self.cats.push_back(animal);
            },
        }
    }

    // Adopts the oldest animal of either species.
    pub fn dequeue_any(&mut self) -> Option<Animal> {
        match(self.dogs.is_empty(), self.cats.is_empty()) {
            (true, true) => None,
            (false, true) =>  self.dequeue_dog(),
            (true, false) =>  self.dequeue_cat(),
            (false, false) => {
                let oldest_animal = self.calc_oldest();
                oldest_animal
            }
        }

    }

    // Adopts the oldest dog.
    pub fn dequeue_dog(&mut self) -> Option<Animal> {
        let dog_to_remove= self.dogs.pop_front();
        self.remove_arrival(dog_to_remove?);

        dog_to_remove
    }

    // Adopts the oldest cat.
    pub fn dequeue_cat(&mut self) -> Option<Animal> {
       let cat_to_remove= self.cats.pop_front();
       self.remove_arrival(cat_to_remove?);

        cat_to_remove
    }

    fn remove_arrival(&mut self, animal_to_remove: Animal) {
        if let Some(i) = self.arrivals.iter().position(|&animal| animal == animal_to_remove) {
            self.arrivals.remove(i);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dogs.is_empty() && self.cats.is_empty()
    }

    pub fn len(&self) -> usize {
        self.dogs.len() + self.cats.len()
    }

    fn calc_oldest(&mut self) -> Option<Animal> {
        match self.arrivals.front()?.species {
            Species::Dog => self.dequeue_dog(),
            Species::Cat => self.dequeue_cat(),
        }
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
