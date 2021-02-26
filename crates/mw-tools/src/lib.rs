#![forbid(unsafe_code)]

mod client;

pub(crate) mod response;

pub mod api;
pub mod error;

pub use client::WikiClient;
