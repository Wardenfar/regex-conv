use std::{fmt::Debug, hash::Hash};

use crate::automata::{Automata, Dfa, State};

pub fn implode_dfa<MF, T, T2>(dfa: &Dfa<T>, n: usize, merge_fn: MF) -> Dfa<T2>
where
    MF: Fn(Vec<T>) -> Vec<T2>,
    T: Clone + Hash + Eq + Debug,
{
    let mut imploded = Automata::new();
    imploded.initial_states = dfa.initial_states.clone();
    imploded.accept_states = dfa.accept_states.clone();

    for from in dfa.all_states() {
        let mut implosions = Vec::new();
        rec_implode(&mut implosions, &mut Vec::new(), dfa, from, n);

        for (symbol_seq, to) in implosions {
            // if the sequence is not full, the merge_fn should return every possible symbol
            // when the mapping is not 1-to-1 (base64, hex, ...)
            let imploded_symbols = merge_fn(symbol_seq);

            for symbol in imploded_symbols {
                imploded.link(from, to, symbol);
            }
        }
    }

    imploded
}

fn rec_implode<T>(
    implosions: &mut Vec<(Vec<T>, State)>,
    curr_seq: &mut Vec<T>,
    dfa: &Dfa<T>,
    from_state: State,
    n: usize,
) where
    T: Clone + Hash + Eq + Debug,
{
    for link in dfa.links_from(from_state) {
        curr_seq.push(link.symbol.clone());

        if dfa.accept_states.contains(&link.to) || n == 1 {
            implosions.push((curr_seq.clone(), link.to));
        }

        if n > 1 {
            rec_implode(implosions, curr_seq, dfa, link.to, n - 1);
        }

        curr_seq.pop();
    }
}
