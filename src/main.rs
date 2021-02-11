pub mod lisp; 
pub mod parser;
pub mod test;
pub mod either;
pub mod eval;

// use test::*;
use lisp::*;
use parser::*;
use eval::*;

fn main() {
    println!("Hello, world!");

    let exp = match Expression::from_str("(/ (+ (* 3 3) 2 (* 1 3) (+ 0 1) 1) (/ 40 2 2 5))") {
    // let exp = match Expression::from_str("(/ (+ (* 3 3) 0 ) 2)") {
        Ok(ex) => ex,
        Err(_) => unreachable!(),
    };

    println!("{:?}", exp);
    // println!("{}", exp.to_string());
    let ev = exp.eval();

    if ev.is_ok() {
        println!("ev {:?}", ev.unwrap());
    }
}
