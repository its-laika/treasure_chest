//! Hash module.
//!
//! This module provides hashing functionalities. It includes submodules
//! for Argon2 hashing and hash definitions.
mod argon2;
mod definitions;

pub use argon2::Argon2 as Hash;
pub use definitions::*;
