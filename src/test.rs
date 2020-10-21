use crate::lisp_descr::Atom;
use std::convert::TryFrom;

pub fn test_atom() {

    ////////// Test booleans
    let bo = Atom::try_from("#t");
    assert_eq!(Ok(Atom::Bool(true)), bo);

    let bo = Atom::try_from("#f");
    assert_eq!(Ok(Atom::Bool(true)), bo);

    let bo = Atom::try_from("#r");
    assert!(bo.is_err());

    ////////// Numbers
    let nu = Atom::try_from("14");
    assert_eq!(Ok(Atom::Number(14)), nu);

    let nu = Atom::try_from("-8123");
    assert_eq!(Ok(Atom::Number(-8123)), nu);

    let nu = Atom::try_from("43e2");
    assert!(nu.is_err());

    ////////// Test chars
    let ch = Atom::try_from("'c'");
    assert_eq!(Ok(Atom::Char('c')), ch);

    let ch = Atom::try_from("'cd'");
    assert!(ch.is_err());

    let ch = Atom::try_from("''");
    assert!(ch.is_err());

    let ch = Atom::try_from("'''");
    assert_eq!(Ok(Atom::Char('\'')), ch);

    ////////// Test strings
    let st = Atom::try_from("\"egunon gazte\"");
    assert_eq!(Ok(Atom::String("egunon gazte".to_string())), st);

    let st = Atom::try_from("\"\"");
    assert_eq!(Ok(Atom::String("".to_string())), st);

    let st = Atom::try_from("\"   \"");
    assert_eq!(Ok(Atom::String("   ".to_string())), st);

    let st = Atom::try_from("\"kaixo\" ");
    assert!(st.is_err());

    let st = Atom::try_from("\"");
    assert!(st.is_err());
}
