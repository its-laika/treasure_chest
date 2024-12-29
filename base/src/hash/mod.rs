//! Hashing module
//! This mod contains the trait and error definitions for hashing (and
//! verifying) data. Also, the implementations for the concrete algorithms are
//! included in this mod.
//!
//! # Definitions
//!
//! * [`Hashing`] for hashing and verifying data
//!
//! # Algorithms
//!
//! * [`Argon2`]
mod argon2;
mod definitions;

pub use argon2::Argon2;
pub use definitions::*;
