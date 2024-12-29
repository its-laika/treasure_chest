//! Base library for rusty_box.
//! This library contains some basic functionality for:
//!
//! * file handling (see [`file`])
//! * encryption / decryption of files (see [`encryption`])
//! * hashing / verification of hashes (see [`hash`])
//! * base64 encoding and decoding (see [`base64`])
//!
//! Most of it are fascades that will simplify using the cargo dependencies.
pub mod base64;
pub mod encryption;
pub mod file;
pub mod hash;
