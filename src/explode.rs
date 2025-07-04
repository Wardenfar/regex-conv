use regex_automata::Dfa;

use crate::codec::Codec;

pub fn explode_dfa<CODEC: Codec>(original_dfa: &Dfa<u8>) -> Result<Dfa<bool>, ()> {
    let counter = original_dfa.next_counter();

    let mut exploded_dfa = Dfa::new();
    exploded_dfa.initial_states = original_dfa.initial_states.clone();
    exploded_dfa.accept_states = original_dfa.accept_states.clone();

    for link in &original_dfa.links {
        let Some(value) = symbol_value::<CODEC>(link.symbol) else {
            println!(
                "symbol {:?} not supported by {:?}",
                link.symbol,
                CODEC::name()
            );
            return Err(());
        };

        let mut prev = link.from;
        for i in 0..CODEC::BITS {
            let is_last = i + 1 == CODEC::BITS;

            let bit_idx = CODEC::BITS - i - 1;
            let bit = ((value >> bit_idx) & 1) != 0;

            let next = if is_last { link.to } else { counter.next() };
            exploded_dfa.link(prev, next, bit);
            prev = next;
        }
    }

    Ok(exploded_dfa)
}

fn symbol_value<CODEC: Codec>(search_sym: u8) -> Option<u32> {
    CODEC::symbols().find_map(|(sym, value)| (sym == search_sym).then_some(value))
}
