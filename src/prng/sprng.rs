#![warn(missing_docs)]
#![allow(non_snake_case)]

//! Crypto Tools - PRNG - GT 2016
//!
//! Module implementing the sponge-based PRNG of Gazi and Tessaro [GT2016].

use std::io::Error;
use rand::{Rng, thread_rng, distributions::Standard, prelude::Distribution};
use std::{ops::{BitXor, BitAnd, BitOr, Not, Sub, Shl}, convert::From};
use crate::prng::PRNG;

#[derive(Clone, Debug)]
/// Structure implementing the Sponge-based PRNG of Gazi and Tessaro [GT2016].
/// Note that the state of the sponge is reversed for easier use of the outputs.
/// The outer part is stored in the lower bits.
pub struct SPRNG<U>
{
    t: usize,           // number of permutation rounds in `next'
    s: usize,           // length of the seed vector'
    j: usize,           // seed iterator
    mask: U,            // mask used to extract the outer part
    perm: fn(U) -> U,   // permutation function
    seed: Vec<U>,       // seed vector
    state: U            // state of the sponge
}

impl<U> SPRNG<U>
    where U: Copy + Clone + From<u8> + Shl<usize, Output = U> + BitAnd<Output = U> +
        Not<Output = U> + BitOr<Output = U> + Sub<Output = U>, Standard: Distribution<U>
{
    /// Setup function.
    pub fn new(params: Vec<usize>, func: fn(U) -> U) -> Result<Self, Error> {
        assert!(params.len() == 4, "SPRNG Setup: wrong number of parameters for setup. Expected 4, got {}.", params.len());
        let (n, r, t, s) = (params[0], params[1], params[2], params[3]);
        assert!(r <= n, "SPRNG Setup: rate r must be less than or equal to the state size n.");
        assert!(s > 1, "SPRNG Setup: seed size s must be greater than 1.");

        // Generate the mask
        let mut mask: U = 1_u8.into();
        mask = (mask << r) - 1_u8.into();

        // Generate the seed using rand
        let mut rng = thread_rng();
        let mut seed_vec: Vec<U> = Vec::with_capacity(s);
        for _ in 0..s {
            seed_vec.push(rng.gen::<U>() & mask);
        }

        // Initial state is r '0' bits and c random bits (n=c+r)
        let mut state: U = 0_u8.into();
        state = state | (rng.gen::<U>() & !mask);

        Ok(SPRNG{
            t: t,
            s: s,
            j: 1_usize,
            mask: mask,
            perm: func,
            seed: seed_vec,
            state: state
        })
    }

    /// Getter for the parameters (t,s).
    pub fn get_params(&self) -> Vec<usize> {
        vec![self.t, self.s]
    }

    /// Getter for the seed.
    pub fn get_seed(&self) -> Vec<U> {
        self.seed.clone()
    }

    /// Getter for the mask.
    pub fn get_mask(&self) -> U {
        self.mask.clone()
    }
}

impl<U> PRNG for SPRNG<U>
    where U: Copy + From<u8> + Not<Output = U> + BitAnd<Output = U> + BitXor<Output = U>
{
    // Here, the state, inputs and outputs are all of the same type U.
    type Input = U;
    type Output = U;

    /// General refresh function.
    fn refresh(&mut self, inputs: Vec<U>) -> Result<(), Error> {
        let l = inputs.len();
        assert!(l > 0, "SPRNG Refresh: no inputs provided.");

        // Refresh the state using all inputs, cycling through the seed vector
        for i in 0..l {
            self.state = (self.perm)(self.state ^
                ((inputs[i] ^ self.seed[self.j]) & self.mask)
            );
            self.j = (self.j + 1) % self.s;
        }

        Ok(())
    }

    /// General next function.
    fn next(&mut self) -> Result<U, Error> {
        // Permute
        self.state = (self.perm)(self.state);

        // Output is the outer part
        let R = self.state & self.mask;

        // Truncate t times
        for _ in 1..self.t {
            self.state = (self.perm)(self.state);
            self.state = self.state & !(self.mask);
        }

        // Reset seed counter
        self.j = 1_u8.into();

        Ok(R)
    }
}