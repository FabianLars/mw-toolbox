#![forbid(unsafe_code)]

mod client;
mod pathtype;

pub(crate) mod response;

pub mod api;
pub mod error;

pub use client::WikiClient;
pub use pathtype::PathType;
