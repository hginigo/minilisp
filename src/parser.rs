use lisp_descr::*;

// Error: position at line.
pub fn split_string(line: &mut String) -> Result<Vec<String>, i32> {
    line.find(LISP_OP);
    unimplemented!();
}

pub fn parse_atom(atom: String) -> Result<Atom, str> {
    match atom.parse::<i32>() {
        Ok(int) => Atom::Number(int),
        _ => {}
    }

    if atom == "#t" {
        Atom::Bool(true)
    }

    if atom == "#f" {
        Atom::Bool(false)
    }

    if atom.len() == 1 {
        Atom::Char(atom.pop())
    }

    // atom.len() != 1
    if atom.starts_with('\"') & atom.ends_with('\"') {
        Atom::String(atom.drain(1..atom.len()).collect())
    }

    Err("The given atom could not be identified")
}
