use std::process::ExitCode;

use clap::{Parser, ValueEnum};

use regex_automata::{determine_and_minimize_nfa, dfa_to_hir, hir_to_nfa};
use regex_conv::{
    codec::{Base64, Bin, Codec, Hex, Raw},
    explode::explode_dfa,
    implode::implode_dfa,
};
use regex_syntax::{
    hir::{Class, ClassBytes, ClassBytesRange, Hir, Repetition},
    ParserBuilder,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    encoding: Encoding,
    regex: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Encoding {
    Raw,
    Base64,
    Hex,
    Bin,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut parser = ParserBuilder::default()
        .dot_matches_new_line(true)
        .unicode(false)
        .utf8(false)
        .build();

    let hir = match parser.parse(&cli.regex) {
        Ok(hir) => hir,
        Err(error) => {
            println!("Regex parsing error : {error}");
            return ExitCode::FAILURE;
        }
    };

    match cli.encoding {
        Encoding::Raw => run::<Raw>(hir),
        Encoding::Base64 => run::<Base64>(hir),
        Encoding::Hex => run::<Hex>(hir),
        Encoding::Bin => run::<Bin>(hir),
    }
}

fn run<C: Codec>(mut hir: Hir) -> ExitCode {
    // for some encoding, like base64, the regex needs to take into account a number of previous char
    let byte_span = C::BITS as u32 / lcm(C::BITS as u32, 8);
    if byte_span > 1 {
        let any_byte = Hir::class(Class::Bytes(ClassBytes::new([ClassBytesRange::new(
            0, 255,
        )])));
        let any_byte_empty = Hir::alternation(vec![any_byte, Hir::empty()]);

        let prefix = Hir::repetition(Repetition {
            min: byte_span - 1,
            max: Some(byte_span - 1),
            greedy: true, // not effectful
            sub: Box::new(any_byte_empty),
        });

        hir = Hir::concat(vec![prefix, hir]);
    }

    let nfa = hir_to_nfa(&hir);
    let dfa = determine_and_minimize_nfa(nfa);

    let Ok(exploded) = explode_dfa::<Raw>(&dfa) else {
        return ExitCode::FAILURE;
    };

    let min_exploded = determine_and_minimize_nfa(exploded.into_nfa());
    let imploded = implode_dfa::<C>(&min_exploded);
    let min_imploded = determine_and_minimize_nfa(imploded.into_nfa());
    let regex = dfa_to_hir(min_imploded);
    println!("{regex}");

    ExitCode::SUCCESS
}

fn lcm(a: u32, b: u32) -> u32 {
    b * gcd(a, b) / a
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while a != 0 {
        let remainder = b % a;
        b = a;
        a = remainder;
    }
    b
}
