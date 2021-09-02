//! A Rust-native WebAssembly syntax model useful for generating, parsing, and emitting WebAssembly code.

pub mod model;
pub use model::*;

#[cfg(feature = "parser")]
pub mod parser;

#[cfg(feature = "parser")]
pub use parser::*;
