use std::str::FromStr;
use std::string;
use std::fmt;
pub use crate::either::*;

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

impl Atom {
    pub fn to_string(self) -> String {
        match self {
            Atom::Bool(val) => if val { String::from("#t") } 
                                else { String::from("#f") },
            Atom::Number(num) => format!("{}", num),
            Atom::Char(c) => format!("'{}'", c),
            Atom::String(st) => format!("\"{}\"", st),
        }
    }
}

// pub type List = Vec<Expression>;

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol {
    symbol: String,
    value: Atom,
}

impl FromStr for Symbol {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = Symbol {
            symbol: s.to_string(),
            // TODO: look in the heap
            value: Atom::Number(1),
        };
        Ok(v)
    }
}

impl Symbol {
    pub fn to_string(self) -> String {
        self.symbol
    }
}

pub type Value = Either<Atom, Symbol>;

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Try to parse an atom
        let res = Atom::from_str(s);
        if res.is_ok() {
            return Ok(Left(res.unwrap()));
        }

        // Else, try to parse a symbol
        let sym = Symbol::from_str(s)?;
        Ok(Right(sym))
    }
}

impl Value {
    pub fn to_string(self) -> String {
        match self {
            Left(atom) => atom.to_string(),
            Right(symb) => symb.to_string(),
        }
    }
}

// They are evaluated
pub type Expression = Either<Value, Compound>;

impl FromStr for Expression {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: esto q es
        let string_of_s = String::from(s);
        let copy_of_s = string_of_s.as_str();

        // An expression is either an atom or a compound.
        match Value::from_str(s) {
            // The expression is an atom, return it.
            Ok(val) => Ok(Expression::Left(val)),
            // The expression is not an atom,
            // try to parse it as a compound
            _ => match Compound::from_str(copy_of_s) {
                Ok(comp) => Ok(Expression::Right(comp)),
                _ => Err("Could not parse the expression"),
            }
        }
    }
}

impl Expression {
    pub fn to_string(self) -> String {
        match self {
            Left(atom) => atom.to_string(),
            Right(comp) => comp.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Compound {
    operator: String,
    operands: Vec<Expression>,
}

fn find_next_subexpr(s: &str) -> Option<usize> {
    if s.starts_with(LISP_OPC) {
        s.find(LISP_CLC).map(|u| u + 1)
    } else {
        s.find(LISP_SEP)
    }
}

impl FromStr for Compound {
    type Err = &'static str;

    // TODO: Minimize trim method calls
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if !(s.starts_with(LISP_OPC) && s.ends_with(LISP_CLC)) {
            return Err("Malformed expression");
        }
        // Since they are two different chars,
        // the length of s has to be more than one
        let mut s = String::from(&s[1..s.len()-1]);

        let op = match s
            .find(|c: char| c == LISP_SEP || c == LISP_OPC) {
                Some(num) => num,
                None => return Err("asd"),
        };

        // operator found
        let operator = s.drain(..op)
            .collect::<String>()
            .trim()
            .to_string();

        // Find the remaining subexpressions and split them
        let mut subexpr: Vec<Expression> = Vec::new();
        let mut s = s.as_str();
        // println!("s:{}:{}:", operator, s);
        
        while !s.is_empty() {
            s = s.trim();
            // println!("s:{}:", s);
            let expr_ind = match find_next_subexpr(s) {
                Some(num) => num,
                // TODO: handle incomplete expressions
                None => s.len(),
            };

            let result = s.split_at(expr_ind);
            let expr = result.0;
            s = result.1;

            // println!("expr:{}:{}:", expr, s);
            let expr = match Expression::from_str(expr) {
                Ok(exp) => exp,
                Err(e) => return Err(e),
            };
            subexpr.push(expr);
            // println!("sub:{}:", s);
        }

        Ok(Compound {
            operator: operator,
            operands: subexpr,
        })
    }
}

impl Compound {
    pub fn to_string(self) -> String {
        let mut res = format!("{}{}",LISP_OPC, self.operator);
        for exp in self.operands {
            res = format!("{}{}{}", res, LISP_SEP, exp.to_string());
        }
        format!("{}{}", res, LISP_CLC)
    }
}

// Definitions
// etc... TODO functions, lambda, let, define

// Keywords
pub const LISP_OPC: char = '(';
pub const LISP_CLC: char = ')';
pub const LISP_SEP: char = ' ';
