pub mod lisp_descr; 
pub mod test;
pub mod either;
use test::*;

fn main() {
    println!("Hello, world!");
    test_atom();
    test_expr();
}
