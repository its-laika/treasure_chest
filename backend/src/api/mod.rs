//! API module.
//!
//! This module contains the routes and server setup for the API. It includes
//! submodules for configuration, download, and upload routes, as well as the
//! server initialization.
mod routes {
    pub mod configuration;
    pub mod download;
    pub mod upload;
}
mod server;
pub use server::listen;
