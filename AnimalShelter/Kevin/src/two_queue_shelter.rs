use std::collections::VecDeque;

use crate::Animal;
use crate::AnimalShelter;
use crate::NamedShelter;
use crate::Species;

#[derive(Debug)]
struct ShelteredAnimal
{
    animal: Animal,
    order: usize,
}

impl ShelteredAnimal
{
    const fn is_older_than(&self, other: &Self) -> bool { self.order < other.order }
}

fn dequeue_animal(queue: &mut VecDeque<ShelteredAnimal>) -> Option<Animal> { queue.pop_front().map(|sheltered| sheltered.animal) }

#[derive(Debug, Default)]
pub struct TwoQueueShelter
{
    dogs: VecDeque<ShelteredAnimal>,
    cats: VecDeque<ShelteredAnimal>,
    next_order: usize,
}

impl TwoQueueShelter
{
    #[must_use]
    pub fn new() -> Self { Self::default() }
}

impl NamedShelter for TwoQueueShelter
{
    const NAME: &'static str = "two_queue_shelter";
}

impl AnimalShelter for TwoQueueShelter
{
    fn enqueue(&mut self, animal: Animal)
    {
        let queue = match animal.species
        {
            Species::Cat => &mut self.cats,
            Species::Dog => &mut self.dogs,
        };
        queue.push_back(ShelteredAnimal { animal, order: self.next_order });
        self.next_order += 1;
    }

    fn dequeue_any(&mut self) -> Option<Animal>
    {
        let queue = match (self.dogs.front(), self.cats.front())
        {
            (None, _) => &mut self.cats,
            (_, None) => &mut self.dogs,
            (Some(dog), Some(cat)) =>
            {
                if dog.is_older_than(cat)
                {
                    &mut self.dogs
                }
                else
                {
                    &mut self.cats
                }
            }
        };
        dequeue_animal(queue)
    }

    fn dequeue_dog(&mut self) -> Option<Animal> { dequeue_animal(&mut self.dogs) }

    fn dequeue_cat(&mut self) -> Option<Animal> { dequeue_animal(&mut self.cats) }

    fn is_empty(&self) -> bool { self.dogs.is_empty() && self.cats.is_empty() }

    fn len(&self) -> usize { self.dogs.len() + self.cats.len() }
}
