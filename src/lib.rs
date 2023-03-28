pub mod error;
pub mod serializer;

mod tag;
pub use tag::*;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}
