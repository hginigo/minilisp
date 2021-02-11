use crate::lisp::*;
use std::string::String;
use std::collections::HashMap;

pub type Heap = HashMap<String, Expression>;

fn sum(list: Vec<Expression>) -> Result<Atom, &'static str> {
    let mut sum = 0;
    for exp in list {
        let res = exp.eval()?;
        let x = match res {
            Atom::Number(x) => x,
            _ => return Err("Not a number"),
        };
        sum += x;
    }
    Ok(Atom::Number(sum))
}

fn sub(list: Vec<Expression>) -> Result<Atom, &'static str> {
    let mut list = list;
    list.reverse();
    match list.len() {
        0 => return Err("No args"),
        1 => match list.pop().unwrap().eval()? {
            Atom::Number(x) => return Ok(Atom::Number(-1 * x)),
            _ => return Err("Not a number"),
        },
        _ => {},
    };

    let mut sub = match list.pop().unwrap().eval()? {
        Atom::Number(x) => x,
        _ => return Err("Not a number"),
    };
    for exp in list {
        let res = exp.eval()?;
        let x = match res {
            Atom::Number(x) => x,
            _ => return Err("Not a number"),
        };
        sub -= x;
    };
    Ok(Atom::Number(sub))
}

fn mul(list: Vec<Expression>) -> Result<Atom, &'static str> {
    let mut mul = 1;
    for exp in list {
        let res = exp.eval()?;
        let x = match res {
            Atom::Number(x) => x,
            _ => return Err("Not a number"),
        };
        mul *= x;
    }
    Ok(Atom::Number(mul))
}

fn div(list: Vec<Expression>) -> Result<Atom, &'static str> {
    let mut list = list;
    list.reverse();
    match list.len() {
        0 => return Err("No args"),
        1 => match list.pop().unwrap().eval()? {
            Atom::Number(x) => return Ok(Atom::Number(x)),
            _ => return Err("Not a number"),
        },
        _ => {},
    };

    let mut div = match list.pop().unwrap().eval()? {
        Atom::Number(x) => x,
        _ => return Err("Not a number"),
    };
    for exp in list {
        let res = exp.eval()?;
        let x = match res {
            Atom::Number(x) => x,
            _ => return Err("Not a number"),
        };
        div /= x;
    };
    Ok(Atom::Number(div))
}

pub trait Eval {
    type Err;
    fn eval(self) -> Result<Atom, Self::Err>;
}

impl Eval for Atom {
    type Err = &'static str;

    fn eval(self) -> Result<Atom, Self::Err> {
        Ok(self)
    }
}

impl Eval for Name {
    type Err = &'static str;

    fn eval(self) -> Result<Atom, Self::Err> {
        Ok(self.value)
    }
}

impl Eval for Value {
    type Err = &'static str;

    fn eval(self) -> Result<Atom, Self::Err> {
        match self {
            Left(atom) => atom.eval(),
            Right(name) => name.eval(),
        }
    }
}

impl Eval for Compound {
    type Err = &'static str;

    fn eval(self) -> Result<Atom, Self::Err> {
        match self.operator.as_str() {
            "+" => sum(self.operands),
            "-" => sub(self.operands),
            "*" => mul(self.operands),
            "/" => div(self.operands),
            _ => Ok(Atom::Number(0)),
        }
    }
}

impl Eval for Expression {
    type Err = &'static str;

    fn eval(self) -> Result<Atom, Self::Err> {
        match self {
            Left(value) => value.eval(),
            Right(comp) => comp.eval(),
        }
    }
}

