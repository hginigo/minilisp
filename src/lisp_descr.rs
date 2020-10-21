pub use either::Either;

// Basic types
pub type LispType = Either<Atom, List>;

pub enum Atom {
    Bool(bool),
    // NOTE: float support needed
    Number(i32),
    Char(char),
    String(String),
}

pub type List = Vec<Atom>;

// Expressions
pub type Expression = Either<Atom, Compound>;

pub struct Compound {
    operator: Atom,
    operands: Vec<Expression>,
}

// Definitions
// etc... TODO functions, lambda, let, define

// Keywords
pub const LispOp: char = '(';
pub const LispCl: char = ')';
