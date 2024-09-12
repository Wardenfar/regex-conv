use clap::Parser;
use itertools::Itertools;
use regex_conv::{
    determine::determine_and_min_nfa, dfa_to_hir::dfa_to_hir, explode::explode_dfa,
    hir_to_nfa::hir_to_nfa, implode::implode_dfa, to_dot::automata_to_dot,
};
use regex_syntax::{
    hir::{Class, ClassBytes, ClassBytesRange, Hir},
    ParserBuilder,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    strict_offset: bool,
    regex: String,
}

fn main() {
    let cli = Cli::parse();

    let mut parser = ParserBuilder::default().unicode(false).utf8(false).build();
    let mut hir = parser.parse(&cli.regex).unwrap();

    if !cli.strict_offset {
        let dot = Hir::class(Class::Bytes(ClassBytes::new([ClassBytesRange::new(
            0, 255,
        )])));
        let dot_opt = Hir::alternation(vec![dot, Hir::empty()]);
        hir = Hir::concat(vec![dot_opt.clone(), dot_opt.clone(), hir]);
    }

    let nfa = hir_to_nfa(&hir);
    // automata_to_dot(&mut stdout(), &nfa).unwrap();

    let dfa = determine_and_min_nfa(nfa);

    // automata_to_dot(&mut stdout(), &dfa).unwrap();

    // dbg!(hir);
    // dbg!(&nfa);
    //nfa_to_dot(&mut stdout(), &nfa).unwrap();
    let exploded = explode_dfa(&dfa, |byte: &u8| {
        (0..8).rev().map(|i| (byte >> i) & 1 == 1).collect_vec()
    });

    let min_exploded = determine_and_min_nfa(exploded.to_nfa());

    // automata_to_dot(&mut stdout(), &min_exploded).unwrap();

    let imploded = implode_dfa(&min_exploded, 6, |list| {
        assert!(list.len() <= 6);
        if list.len() == 0 {
            return Vec::new();
        }
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let mut ones = 0_u8;
        let mut zeroes = 0_u8;
        for bit in 0..6 {
            zeroes <<= 1;
            ones <<= 1;
            match list.get(bit) {
                Some(true) => {
                    ones |= 1;
                    zeroes |= 1;
                }
                Some(false) => {}
                None => {
                    ones |= 1;
                }
            }
        }

        alphabet
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                let idx = idx as u8;
                let mask = !(zeroes ^ ones);
                if idx & mask == zeroes & mask {
                    Some(c)
                } else {
                    None
                }
            })
            .collect_vec()
    });

    //dbg!(&nfa3);

    let min_imploded = determine_and_min_nfa(imploded.to_nfa());
    //automata_to_dot(&mut stdout(), &min_imploded).unwrap();

    let regex = dfa_to_hir(&min_imploded);
    println!("{regex}")
}
