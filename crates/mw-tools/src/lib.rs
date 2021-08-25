#![forbid(unsafe_code)]

pub use client::Client;
pub use error::Error;

mod client;
mod error;

pub mod api;
pub mod response;
