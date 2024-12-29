//! Encryption module
//! This mod contains the trait and error definitions for encryption (and
//! encoding of encrypted data). Also, the implementations for the concrete
//! algorithms are included in this mod.
//!
//! # Definitions
//!
//! * [`Encoding`] for encoding encrypted data with everything
//!   necessary to decrypt it later.
//! * [`Encryption`] for actually encrypting and decrypting the
//!   data.
//!
//! # Algorithms
//!
//! * [`XChaCha20Poly1305`]

mod definitions;
mod xchacha20poly1305;

pub use definitions::*;
pub use xchacha20poly1305::EncryptionData as XChaCha20Poly1305;
