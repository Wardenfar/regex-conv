use regex_syntax::hir::{Class, Hir, HirKind, Literal};

use crate::{
    automata::{Automata, MaybeSymbol::*, Nfa, State},
    counter::Counter,
};

pub fn hir_to_nfa(hir: &Hir) -> Nfa<u8> {
    let mut nfa = Automata::new();
    let mut counter = Counter::new(0);
    let (start, end) = rec_hir_to_nfa(hir, &mut counter, &mut nfa);
    nfa.initial_states.insert(start);
    nfa.accept_states.insert(end);
    nfa
}

fn rec_hir_to_nfa(hir: &Hir, counter: &mut Counter, nfa: &mut Nfa<u8>) -> (State, State) {
    let start = counter.next();
    let end = counter.next();
    match hir.kind() {
        HirKind::Empty => {
            nfa.link(start, end, Epsilon);
        }
        HirKind::Literal(Literal(bytes)) => {
            let mut prev = start;
            for byte in bytes.iter() {
                let next = counter.next();
                nfa.link(prev, next, Symbol(*byte));
                prev = next;
            }
            nfa.link(prev, end, Epsilon);
        }
        HirKind::Class(class) => match class {
            Class::Unicode(class) => {
                let iter = class.ranges().iter().flat_map(|range| {
                    (range.start()..=range.end()).map(|c| {
                        let mut bytes = [0; 4];
                        c.encode_utf8(&mut bytes);
                        (c.len_utf8(), bytes)
                    })
                });

                let mut prev_len = 0;
                let mut prev_bytes = [0; 4];
                let mut common_states: [State; 4] = [0; 4];
                let mut common_len = 0;
                for (len, bytes) in iter {
                    assert!(prev_len <= 4);
                    assert!(1 <= len && len <= 4);

                    let mut prev_state = start;
                    for i in 0..len {
                        let prev_byte = prev_bytes[i];
                        let byte = bytes[i];

                        if i >= common_len || prev_byte != byte {
                            common_len = i;

                            let next_state = counter.next();
                            common_states[i] = next_state;

                            nfa.link(prev_state, next_state, Symbol(byte));
                            prev_state = next_state;
                        } else {
                            prev_state = common_states[i];
                        }
                    }

                    nfa.link(prev_state, end, Epsilon);
                    prev_len = len;
                    common_len = len;
                    prev_bytes = bytes;
                }
            }
            Class::Bytes(class) => {
                for range in class.ranges() {
                    for byte in range.start()..=range.end() {
                        nfa.link(start, end, Symbol(byte));
                    }
                }
            }
        },
        HirKind::Look(_) => todo!(),
        HirKind::Repetition(repetition) => {
            let mut prev = start;
            for _ in 0..repetition.min {
                let (item_start, item_end) = rec_hir_to_nfa(&repetition.sub, counter, nfa);
                nfa.link(prev, item_start, Epsilon);
                prev = item_end;
            }

            if let Some(max) = repetition.max {
                for _ in repetition.min..max {
                    let (item_start, item_end) = rec_hir_to_nfa(&repetition.sub, counter, nfa);
                    nfa.link(prev, item_start, Epsilon);
                    nfa.link(prev, end, Epsilon);
                    prev = item_end;
                }
            } else {
                let (item_start, item_end) = rec_hir_to_nfa(&repetition.sub, counter, nfa);
                nfa.link(prev, item_start, Epsilon);
                nfa.link(item_end, item_start, Epsilon);
                nfa.link(item_end, end, Epsilon);
            }
            nfa.link(prev, end, Epsilon);
        }
        HirKind::Capture(capture) => {
            let (item_start, item_end) = rec_hir_to_nfa(&capture.sub, counter, nfa);
            nfa.link(start, item_start, Epsilon);
            nfa.link(item_end, end, Epsilon);
        }
        HirKind::Concat(list) => {
            let mut prev = start;
            for item in list {
                let (item_start, item_end) = rec_hir_to_nfa(item, counter, nfa);
                nfa.link(prev, item_start, Epsilon);
                prev = item_end;
            }
            nfa.link(prev, end, Epsilon);
        }
        HirKind::Alternation(list) => {
            for item in list {
                let (item_start, item_end) = rec_hir_to_nfa(item, counter, nfa);
                nfa.link(start, item_start, Epsilon);
                nfa.link(item_end, end, Epsilon);
            }
        }
    }
    (start, end)
}
