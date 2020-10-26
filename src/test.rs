use crate::lisp_descr::*;
use std::str::FromStr;

pub fn test_atom() {
    ////////// Test booleans
    let bo = Atom::from_str("#t");
    assert_eq!(Ok(Atom::Bool(true)), bo);

    let bo = Atom::from_str("#f");
    assert_eq!(Ok(Atom::Bool(true)), bo);

    let bo = Atom::from_str("#r");
    assert!(bo.is_err());

    ////////// Numbers
    let nu = Atom::from_str("14");
    assert_eq!(Ok(Atom::Number(14)), nu);

    let nu = Atom::from_str("-8123");
    assert_eq!(Ok(Atom::Number(-8123)), nu);

    let nu = Atom::from_str("43e2");
    assert!(nu.is_err());

    ////////// Test chars
    let ch = Atom::from_str("'c'");
    assert_eq!(Ok(Atom::Char('c')), ch);

    let ch = Atom::from_str("'cd'");
    assert!(ch.is_err());

    let ch = Atom::from_str("''");
    assert!(ch.is_err());

    let ch = Atom::from_str("'''");
    assert_eq!(Ok(Atom::Char('\'')), ch);

    ////////// Test strings
    let st = Atom::from_str("\"egunon gazte\"");
    assert_eq!(Ok(Atom::String("egunon gazte".to_string())), st);

    let st = Atom::from_str("\"\"");
    assert_eq!(Ok(Atom::String("".to_string())), st);

    let st = Atom::from_str("\"   \"");
    assert_eq!(Ok(Atom::String("   ".to_string())), st);

    let st = Atom::from_str("\"kaixo\" ");
    assert!(st.is_err());

    let st = Atom::from_str("\"");
    assert!(st.is_err());
}

pub fn test_expr() {
    let ex = Expression::from_str("(sum \"holaa\" 'c' #t  1     2 3 12 43 -123)");
    assert!(ex.is_ok());

    let ex = Expression::from_str("1");
    assert_eq!(ex, Ok(Expression::Left(Atom::Number(1))));

    let ex = Expression::from_str("(+ 1 2)");
    assert!(ex.is_ok());

    let ex = Expression::from_str("(+ 1 (+ 1 \"asd\") 2)");
    assert!(ex.is_ok());

}
