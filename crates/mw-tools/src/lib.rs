#![forbid(unsafe_code)]

pub use client::WikiClient;

mod client;

pub(crate) mod response;

pub mod api;
pub mod error;
