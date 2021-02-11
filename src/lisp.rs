pub use crate::either::*;
pub use Atom::{Bool, Number, Char, String};
use std::string;

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    Bool(bool),
    // NOTE: float support needed
    Number(i32),
    Char(char),
    // NOTE: use str instead
    String(string::String),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Name {
    pub name: string::String,
    pub value: Atom,
}

pub type Value = Either<Atom, Name>;

// They are evaluated
pub type Expression = Either<Value, Compound>;

#[derive(Debug, Eq, PartialEq)]
pub struct Compound {
    pub operator: string::String,
    pub operands: Vec<Expression>,
}

// Definitions
// etc... TODO functions, lambda, let, define

// Keywords
pub const LISP_OPC: char = '(';
pub const LISP_CLC: char = ')';
pub const LISP_SEP: char = ' ';

