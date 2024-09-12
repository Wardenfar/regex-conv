use itertools::Itertools;
use regex_syntax::hir::{Class, ClassUnicode, ClassUnicodeRange, Hir, Repetition};

use crate::{
    automata::{Automata, Dfa, Link},
    counter::Counter,
};

pub fn dfa_to_hir(dfa_origin: &Dfa<char>) -> Hir {
    let mut dfa = Dfa {
        accept_states: Default::default(),
        initial_states: Default::default(),
        links: dfa_origin
            .links
            .iter()
            .map(|link| Link {
                from: link.from,
                to: link.to,
                symbol: Hir::class(Class::Unicode(ClassUnicode::new([ClassUnicodeRange::new(
                    link.symbol,
                    link.symbol,
                )]))),
            })
            .collect(),
    };

    let mut counter = Counter::new(dfa_origin.max_state() + 1);
    let start = counter.next();
    let end = counter.next();

    for init_state in &dfa_origin.initial_states {
        dfa.link(start, *init_state, Hir::empty());
    }

    for accept_state in &dfa_origin.accept_states {
        dfa.link(*accept_state, end, Hir::empty());
    }

    merge_sibling_edges(&mut dfa);

    let mut all_states = dfa.all_states();
    all_states.remove(&start);
    all_states.remove(&end);

    for rip in all_states {
        debug_assert!(dfa.links_from_to(rip, rip).count() <= 1);

        let self_loop = dfa.links_from_to(rip, rip).exactly_one().ok();
        let self_loop = self_loop.map(|self_loop| {
            Hir::repetition(Repetition {
                greedy: false,
                min: 0,
                max: None,
                sub: Box::new(self_loop.symbol.clone()),
            })
        });

        let incomings_groups = dfa.links_to(rip).cloned().into_group_map_by(|x| x.from);
        let outgoings_groups = dfa.links_from(rip).cloned().into_group_map_by(|x| x.to);

        dfa.remove_links_any(rip);

        debug_assert_eq!(dfa.links_from_to(rip, rip).count(), 0);

        for (from, incomings) in &incomings_groups {
            for (to, outgoings) in &outgoings_groups {
                if *from == rip || *to == rip {
                    continue;
                }
                debug_assert_ne!(from, to);

                let in_sym = Hir::alternation(incomings.iter().map(|l| l.symbol.clone()).collect());
                let out_sym =
                    Hir::alternation(outgoings.iter().map(|l| l.symbol.clone()).collect());

                let hir = if let Some(self_loop) = self_loop.as_ref().cloned() {
                    Hir::concat(vec![in_sym, self_loop, out_sym])
                } else {
                    Hir::concat(vec![in_sym, out_sym])
                };

                dfa.link(*from, *to, hir);
            }
        }

        merge_sibling_edges(&mut dfa);

        debug_assert_eq!(dfa.links_from_to(rip, rip).count(), 0);
    }

    assert_eq!(dfa.links.len(), 1);

    dfa.links.remove(0).symbol
}

fn merge_sibling_edges(dfa: &mut Automata<Hir>) {
    for states in dfa.all_states().iter().permutations(2) {
        let [a, b] = states.as_slice() else { panic!() };
        let from = **a;
        let to = **b;

        let hirs = dfa
            .links_from_to(from, to)
            .map(|l| l.symbol.clone())
            .collect_vec();

        if hirs.len() <= 1 {
            continue;
        }

        dfa.remove_links(from, to);

        dfa.link(from, to, Hir::alternation(hirs));
    }
}
