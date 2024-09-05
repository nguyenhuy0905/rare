#![warn(clippy::all)]
pub mod rare;
pub use rare::RARE;

mod parser;
mod lexer;
mod postfix_converter;
