//! Encryption module.
//!
//! This module provides encryption functionalities. It includes submodules
//! for encryption definitions. Currently this contains the XChaCha20Poly1305
//! encryption scheme.
pub(crate) mod definitions;
mod xchacha20poly1305;

pub use definitions::*;
pub use xchacha20poly1305::XChaCha20Poly1305Data as Data;
