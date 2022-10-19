use crate::rand;

/// A crab's diet: fish, shellfish, or plants (seaweed, algae, etc).
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Diet {
    Fish,
    Shellfish,
    Plants,
}

impl Diet {
    pub fn random_diet() -> Diet {
        // This brings the names in Diet into scope, so we can write
        // `Fish` rather than `Diet::Fish` (and so on) below.
        use Diet::*;
        let r: u32 = rand::rand32() % 3;
        match r {
            0 => Fish,
            1 => Shellfish,
            2 => Plants,
            _ => unreachable!(),
        }
    }
}
