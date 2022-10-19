use crate::prey::Prey;

// VecDeque is Rust's implementation of a double-ended queue, and
// is used only if we only need to use it in a single-ended manner.
use std::collections::vec_deque::{Iter, VecDeque};

#[derive(Debug)]
pub struct Reef {
    prey: VecDeque<Box<dyn Prey>>,
}

impl Reef {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn prey(&self) -> Iter<Box<dyn Prey>> {
        unimplemented!();
    }

    pub fn population(&self) -> usize {
        unimplemented!();
    }

    /**
     * Adds a prey to the reef.
     *
     * This function takes ownership of the boxed prey.
     */
    pub fn add_prey(&mut self, prey: Box<dyn Prey>) {
        unimplemented!();
    }

    /**
     * Returns the next available prey.
     *
     * The callee of this function receives ownership of the boxed prey.
     */
    pub fn take_prey(&mut self) -> Option<Box<dyn Prey>> {
        unimplemented!();
    }
}
