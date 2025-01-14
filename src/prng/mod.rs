#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - PRNG
//!
//! This module groups all Pseudo Random Number Generators.
//! These are accessible through the PRNG trait, which implements the `refresh' and `next' functions.

use std::io::Error;

/// Trait for Pseudo Random Number Generators,
/// with public general-purpose functions `refresh' and `next'.
pub trait PRNG
{
    /// Type of the input to the PRNG.
    type Input;
    /// Type of the output of the PRNG.
    type Output;

    /// General `refresh' function.
    /// Reseed the state of the PRNG using the given inputs.
    fn refresh(&mut self, inputs: Vec<Self::Input>) -> Result<(), Error>;

    /// General `next' function.
    /// Compute the next output of the PRNG.
    fn next(&mut self) -> Result<Self::Output, Error>;
}

/// Module implementing the Sponge-based PRNG of Gazi and Tessaro [GT2016].
pub mod sprng;