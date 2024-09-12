use itertools::{Itertools, Position};

use crate::{automata::Dfa, counter::Counter};

pub fn explode_dfa<EF, T, T2>(dfa: &Dfa<T>, expand_fn: EF) -> Dfa<T2>
where
    EF: Fn(&T) -> Vec<T2>,
{
    let mut counter = Counter::new(dfa.max_state() + 1);

    let mut exploded = Dfa::new();
    exploded.initial_states = dfa.initial_states.clone();
    exploded.accept_states = dfa.accept_states.clone();

    for link in &dfa.links {
        let expanded_symbol = expand_fn(&link.symbol);

        let mut prev = link.from;
        for (position, item) in expanded_symbol.into_iter().with_position() {
            let next = match position {
                Position::Last | Position::Only => link.to,
                Position::Middle | Position::First => counter.next(),
            };
            exploded.link(prev, next, item);
            prev = next;
        }
    }

    exploded
}
