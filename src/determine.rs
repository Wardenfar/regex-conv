use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use crate::{
    automata::{Dfa, MaybeSymbol, Nfa, State},
    counter::Counter,
};

/// A multi state is a collection of NFA states mapping to one DFA state
///
/// [BTreeSet] because [HashSet] doesn't impl [Hash]
type MultiState = BTreeSet<State>;

pub fn determine_and_min_nfa<T>(mut nfa: Nfa<T>) -> Dfa<T>
where
    T: Eq + Hash + Clone + Debug,
{
    nfa.invert();
    let mut dfa = determine_nfa(&nfa);
    dfa.invert();
    let nfa2 = dfa.to_nfa();
    determine_nfa(&nfa2)
}

fn determine_nfa<T>(nfa: &Nfa<T>) -> Dfa<T>
where
    T: Eq + Hash + Clone + Debug,
{
    let mut dfa = Dfa::new();
    let mut counter = Counter::new(0);
    let mut state_mapping: HashMap<MultiState, State> = Default::default();

    macro_rules! nfa2dfa {
        ($multi_state:expr) => {
            *state_mapping
                .entry($multi_state.clone())
                .or_insert_with(|| {
                    let next = counter.next();
                    let is_accept = $multi_state
                        .iter()
                        .any(|state| nfa.accept_states.contains(state));
                    if is_accept {
                        dfa.accept_states.insert(next);
                    }
                    next
                })
        };
    }

    let initial_state = nfa.initial_states.iter().copied().collect();
    let initial_state = normalize_multi_state(nfa, initial_state);
    dfa.initial_states.insert(nfa2dfa!(initial_state));

    let mut to_explore: HashSet<MultiState> = Default::default();
    let mut explored: HashSet<MultiState> = Default::default();

    to_explore.insert(initial_state.clone());
    explored.insert(initial_state);

    while !to_explore.is_empty() {
        for norm_from in std::mem::take(&mut to_explore) {
            let mut to_by_symbol: HashMap<T, MultiState> = HashMap::new();

            for link in &nfa.links {
                if !norm_from.contains(&link.from) {
                    continue;
                }

                let MaybeSymbol::Symbol(symbol) = &link.symbol else {
                    continue;
                };

                to_by_symbol
                    .entry(symbol.clone())
                    .or_default()
                    .insert(link.to);
            }

            let dfa_from = nfa2dfa!(norm_from);

            for (symbol, to) in to_by_symbol {
                let norm_to = normalize_multi_state(nfa, to);
                let dfa_to = nfa2dfa!(norm_to);
                dfa.link(dfa_from, dfa_to, symbol);

                if explored.contains(&norm_to) {
                    continue;
                }
                explored.insert(norm_to.clone());
                to_explore.insert(norm_to.clone());
            }
        }
    }

    dfa
}

fn normalize_multi_state<T>(nfa: &Nfa<T>, from: MultiState) -> MultiState {
    let mut to_explore = from.clone();
    let mut explored = MultiState::new();
    let mut result = MultiState::new();

    while !to_explore.is_empty() {
        for state in std::mem::take(&mut to_explore) {
            if explored.contains(&state) {
                continue;
            }
            result.insert(state);
            explored.insert(state);
            for link in nfa.links_from(state) {
                if let MaybeSymbol::Epsilon = &link.symbol {
                    to_explore.insert(link.to);
                }
            }
        }
    }

    result
}
