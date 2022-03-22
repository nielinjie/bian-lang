
extern crate nom;

pub mod ast;
pub mod parsers;
pub mod wasm;
fn main() {}

#[derive( Debug,Clone,PartialEq)]
pub enum Error{
    ParseError(String),
    CompileError(String),
    RuntimeError(String),
}

#[cfg(test)]
mod test;

