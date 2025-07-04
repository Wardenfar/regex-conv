use regex_automata::{Automata, Dfa, State};

use crate::codec::Codec;

pub fn implode_dfa<CODEC: Codec>(original_dfa: &Dfa<bool>) -> Dfa<u8> {
    let mut imploded = Automata::new();
    imploded.initial_states = original_dfa.initial_states.clone();
    imploded.accept_states = original_dfa.accept_states.clone();

    let mut to_visit = imploded.initial_states.clone();
    let mut visited = imploded.accept_states.clone();

    while !to_visit.is_empty() {
        let loop_to_visit = core::mem::take(&mut to_visit);
        visited.extend(&loop_to_visit);

        for from in loop_to_visit {
            let mut patterns = Vec::new();
            rec_implode::<CODEC>(original_dfa, from, Pattern::default(), 0, &mut patterns);

            for (pattern, to) in patterns {
                for (symbol, value) in CODEC::symbols() {
                    if value & pattern.mask == pattern.value {
                        imploded.link(from, to, symbol);

                        if !visited.contains(&to) {
                            to_visit.insert(to);
                        }
                    }
                }
            }
        }
    }

    imploded
}

#[derive(Debug, Default, Clone, Copy)]
struct Pattern {
    pub mask: u32,
    pub value: u32,
}

fn rec_implode<CODEC: Codec>(
    original_dfa: &Dfa<bool>,
    from_state: State,
    prev_pattern: Pattern,
    depth: usize,
    patterns: &mut Vec<(Pattern, State)>,
) {
    for link in original_dfa.links_from(from_state) {
        let mut link_pattern = prev_pattern;

        link_pattern.value <<= 1;
        link_pattern.mask <<= 1;

        let bit = if link.symbol { 1 } else { 0 };
        link_pattern.value |= bit;
        link_pattern.mask |= 1;

        let is_final = depth + 1 == CODEC::BITS;
        let accept_state = original_dfa.accept_states.contains(&link.to);

        if is_final || accept_state {
            let mut accept_pattern = link_pattern;
            let rest = CODEC::BITS - depth - 1;
            accept_pattern.mask <<= rest;
            accept_pattern.value <<= rest;
            patterns.push((accept_pattern, link.to));
        } else {
            rec_implode::<CODEC>(original_dfa, link.to, link_pattern, depth + 1, patterns);
        }
    }
}
