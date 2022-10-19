use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
}

impl Ocean {
    pub fn new() -> Ocean {
        unimplemented!();
    }

    pub fn add_beach(&mut self, beach: Beach) {
        unimplemented!();
    }

    pub fn beaches(&self) -> Iter<Beach> {
        unimplemented!();
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        unimplemented!();
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        unimplemented!();
    }
}
