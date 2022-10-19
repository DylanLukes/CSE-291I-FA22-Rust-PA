//! This file provides a _deterministic_ source of random numbers for this assignment.
//!   - This is critical to making testing work consistently.
//!   - DO NOT MODIFY THIS FILE.

use rand::RngCore;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use std::cell::RefCell;

thread_local!(
    pub static RNG: RefCell<Pcg64> = RefCell::new(Pcg64::seed_from_u64(0));
);

/**
 * A random number generating function for this assignment.
 *
 * This function has a fixed seed, so it will produce the same sequence of
 * random numbers any time. This is useful for making testing work consistently.
 */
pub fn rand32() -> u32 {
    RNG.with(|r| (*r.borrow_mut()).next_u32())
}
