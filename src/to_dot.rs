use crate::automata::{Automata, Link};
use std::{fmt::Display, io};

pub fn automata_to_dot<T: Display>(
    out: &mut impl io::Write,
    automata: &Automata<T>,
) -> io::Result<()> {
    writeln!(out, "digraph {{")?;

    for init in &automata.initial_states {
        writeln!(out, "s{init} [label=\"init_{init}\"]")?;
    }

    for accept in &automata.accept_states {
        writeln!(out, "s{accept} [label=\"accept_{accept}\"]")?;
    }

    for link in &automata.links {
        let Link { from, symbol, to } = link;
        writeln!(out, "s{from} -> s{to} [label=\"{symbol}\"]")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}
