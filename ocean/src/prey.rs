use crate::crab::Crab;
use crate::diet::Diet;

pub trait Prey {
    /** What diet does this `Prey` fit into? */
    fn diet(&self) -> Diet;

    /**
     * `Prey` are eaten by `Crab`s. This method is called when a crab tries to
     * eat this prey. Return true if the prey gets away, and false if it does not.
     *
     * See the implementations below for some examples of different behaviors.
     */
    fn try_escape(&mut self, crab: &Crab) -> bool;
}

impl core::fmt::Debug for dyn Prey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Prey")
    }
}

#[derive(Debug)]
pub struct Shrimp {
    energy: u32,
}

impl Shrimp {
    pub fn new(energy: u32) -> Shrimp {
        Shrimp { energy }
    }
}

impl Prey for Shrimp {
    fn diet(&self) -> Diet {
        Diet::Shellfish
    }

    /**
     * Shrimp move in bursts, and can escape from crabs if they have enough energy.
     */
    fn try_escape(&mut self, _crab: &Crab) -> bool {
        if self.energy == 0 {
            // No remaining energy to escape!
            return false;
        } else {
            // Escaped, but getting more tired...
            self.energy -= 1;
            return true;
        }
    }
}

#[derive(Debug)]
pub struct Minnow {
    speed: u32,
}

impl Minnow {
    pub fn new(speed: u32) -> Minnow {
        Minnow { speed }
    }
}

impl Prey for Minnow {
    fn diet(&self) -> Diet {
        Diet::Fish
    }

    /**
     * Minnows are fast and tireless, and can always escape from crabs faster than them.
     */
    fn try_escape(&mut self, crab: &Crab) -> bool {
        self.speed > crab.speed()
    }
}

#[derive(Debug)]
pub struct Algae {}

impl Algae {
    pub fn new() -> Algae {
        Algae {}
    }
}

impl Prey for Algae {
    fn diet(&self) -> Diet {
        Diet::Plants
    }

    /**
     * Algae can't move. They're plants.
     */
    fn try_escape(&mut self, _crab: &Crab) -> bool {
        return false;
    }
}

#[derive(Debug)]
pub struct Clam {}

impl Clam {
    pub fn new() -> Clam {
        Clam {}
    }
}

impl Prey for Clam {
    fn diet(&self) -> Diet {
        Diet::Shellfish
    }

    /**
     * There is no escape for the clam.
     */
    fn try_escape(&mut self, _crab: &Crab) -> bool {
        return false;
    }
}
