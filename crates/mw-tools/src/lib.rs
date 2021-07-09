#![forbid(unsafe_code)]

pub use client::WikiClient;

mod client;

pub mod api;
pub mod error;
pub mod response;
