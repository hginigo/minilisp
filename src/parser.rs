pub use std::str::FromStr;
use std::string::String;
use crate::lisp::*;
// use std::fmt;

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
            return Err("Unknown atom.");
        }

        let mut s = String::from(s);
        //println!("s: {}", s);

        let la = s.pop().expect("Unexpected error occurred.");
        let fi = s.remove(0);
        //println!("fi: {}, s: {}, la: {}", fi, s, la);

        if !la.eq(&fi) {
            match s.len() {
                1 => { return Err("Char not closed correctly."); },
                _ => { return Err("String not closed correctly."); },
            }
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
            _ => Err("Unknown string closure"),
        }
    }
}

impl FromStr for Name {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = Name {
            name: s.to_string(),
            // TODO: look inside the heap
            value: Atom::Number(1),
        };
        Ok(n)
    }
}

impl FromStr for Value {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // Try to parse an atom
        let res = Atom::from_str(s);
        if res.is_ok() {
            return Ok(Left(res.unwrap()));
        }

        // Else, try to parse a name
        let name = Name::from_str(s)?;
        Ok(Right(name))
    }
}

impl FromStr for Expression {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // An expression is either an atom or a compound.
        let comp = Compound::from_str(s);
        if comp.is_ok() {
            return Ok(Expression::Right(comp.unwrap()));
        }
        match Value::from_str(s) {
            Ok(val) => Ok(Expression::Left(val)),
            Err(e) => Err(e),
        }
    }
}

// Tries to find the next token for compound lists and returns
// the position where the token ends.
// It does not validate which characters are valid.
fn find_next_token(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let slen = s.len();
    if !s.starts_with(LISP_OPC) {
        return match s.find(LISP_SEP) {
            Some(n) => n,
            None => slen,
        };
    }

    let mut count: i32 = 0;

    for (i, c) in s.chars().enumerate() {
        if c == LISP_OPC {
            count += 1;
        }
        if c == LISP_CLC {
            count -= 1;
            if count == 0 {
                return i+1;
            }
        }
    }
    slen
}

impl FromStr for Compound {
    type Err = &'static str;

    // TODO: Check empty lists
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if !(s.starts_with(LISP_OPC) && s.ends_with(LISP_CLC)) {
            return Err("Malformed expression");
        }
        let s = s[1..s.len()-1].trim();

        let op = match s.find(|c:char| c == LISP_SEP || c == LISP_OPC) {
            Some(n) => n,
            None => s.len(),
        };

        let operator = String::from(&s[0..op]);

        let mut s = s[op..s.len()].trim();
        let mut operands = Vec::new();
        // println!("s:{}:{}:", operator, s);

        while !s.is_empty() {
            let sub = find_next_token(s);

            let operand = match Expression::from_str(&s[0..sub]) {
                Ok(exp) => exp,
                Err(err) => return Err(err),
            };

            operands.push(operand);
            s = &s[sub..s.len()].trim();
            // println!("s:{}:", s);
        }

        Ok(Compound {
            operator: operator,
            operands: operands,
        })
    }
}

