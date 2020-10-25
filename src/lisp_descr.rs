pub use either::*;
use std::str::FromStr;
use std::string;

// Basic types
//pub type LispValue = Either<Expression, List>;

#[derive(Debug, PartialEq, Eq)]
pub enum Atom {
    Bool(bool),
    // NOTE: float support needed
    Number(i32),
    Char(char),
    // NOTE: use str instead
    String(string::String),
}

impl FromStr for Atom {
    //impl Atom {
    // Try to convert the given str into an Atom
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
                    return Err("Too many chars to parse.");
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
// They are evaluated
#[derive(Debug, Eq, PartialEq)]
pub struct Expression(Either<Atom, Compound>);

impl FromStr for Expression {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Hau sekulako basura da ta eztaka bate zentzuik.
        let string_of_s = String::from(s);
        let copy_of_s = string_of_s.as_str();

        // An expression is either an atom or a compound.
        match Atom::from_str(s) {
            // The expression is an atom, return it.
            Ok(atom) => Ok(Expression(Left(atom))),
            // The expression is not an atom,
            // try to parse it as a compound
            _ => match Compound::from_str(copy_of_s) {
                Ok(comp) => Ok(Expression(Right(comp))),
                _ => Err("Could not parse the expression"),
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Compound {
    operator: String,
    operands: Vec<Expression>,
}

impl FromStr for Compound {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if !(s.starts_with(LISP_OPC) || s.ends_with(LISP_CLC)) {
            return Err("Malformed expression");
        }
        // Since they are two different chars,
        // the length of s has to be more than one.
        let mut s = String::from(&s[1..s.len()-1]);

        // operator found
        let op = match s
            .find(|c: char| c == LISP_SEP || c == LISP_OPC) {
                Some(num) => num,
                None => return Err("asd"),
        };

        let operator = s.drain(..op)
            .collect::<String>()
            .trim()
            .to_string();

        // Find the remaining subexpressions and split them
        // TODO: Do this WRIGHT

        let subexpr: Vec<Expression> = s.split(LISP_SEP)
            .filter(|s| !s.is_empty())
            .map(|s: &str| Expression::from_str(s).expect(s))
            .collect();

        Ok(Compound {
            operator: operator,
            operands: subexpr,
        })
    }
}
// Definitions
// etc... TODO functions, lambda, let, define

// Keywords
pub const LISP_OPC: char = '(';
pub const LISP_CLC: char = ')';
pub const LISP_SEP: char = ' ';
