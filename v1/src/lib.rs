// Enforces no-std to prevents accidental heap usage:
#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;


pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

pinocchio_pubkey::declare_id!("FqqrXWzPLkYZsg2r4GfMdzWSdYHorCdkyhupmxRXAzVq");

/*
The difference between solana-program and pinocchio
The main difference and optimization stand in how the entrypoint() behaves.

The Standard Solana entrypoints use traditional serialization patterns where the runtime deserializes input data upfront, creating owned data structures in memory. This approach uses Borsh serialization extensively, copies data during deserialization, and allocates memory for structured data types.
Pinocchio entrypoints implement zero-copy operations by reading data directly from the input byte array without copying. The framework defines zero-copy types that reference the original data, eliminates serialization/deserialization overhead, and uses direct memory access to avoid abstraction layers.

*/