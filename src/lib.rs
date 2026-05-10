#![warn(clippy::all)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod core;
pub mod deserialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod rand;
pub mod serialiser;

#[cfg(test)]
pub mod tests;

pub use core::GDError;