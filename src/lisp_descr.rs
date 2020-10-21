pub use either::Either;
use std::convert::TryFrom;
use std::string;

// Basic types
pub type LispValue = Either<Expression, List>;

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    Bool(bool),
    // NOTE: float support needed
    Number(i32),
    Char(char),
    // NOTE: use str instead
    String(string::String),
}

impl TryFrom<&str> for Atom {
//impl Atom {
    // Try to convert the given str into an Atom
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        // Bool
        if s.eq("#t") {
            return Ok(Atom::Bool(true));
        } else if s.eq("#f") {
            return Ok(Atom::Bool(true));
        }

        // Number
        match s.parse::<i32>() {
            Ok(num) => return Ok(Atom::Number(num)),
            _ => {}
        }

        // Char and String
        if s.len() < 2 {
            return Err("Unknown identifier.");
        }

        let mut s = String::from(s);
        //println!("s: {}", s);

        let la = s.pop().expect("Unexpected error occurred.");
        let fi = s.remove(0);
        //println!("fi: {}, s: {}, la: {}", fi, s, la);

        if !la.eq(&fi) {
            return Err("String or Char not closed correctly.");
        }

        match fi {
            '"' => Ok(Atom::String(s)),
            '\'' => {
                if s.len() > 1 {
                    return Err("Too many chars to parse.") 
                }
                match s.chars().next() {
                    None => Err("Cannot parse empty char."),
                    Some(c) => Ok(Atom::Char(c)),
                }
            }
            _ => Err("Unknown identifier"),
        }
    }
}

pub type List = Vec<Expression>;

// Expressions
pub type Expression = Either<Atom, Compound>;

pub struct Compound {
    operator: String,
    operands: Vec<Expression>,
}

// Definitions
// etc... TODO functions, lambda, let, define

// Keywords
pub const LISP_OPC: char = '(';
pub const LISP_CLC: char = ')';
pub const LISP_SEP: char = ' ';
