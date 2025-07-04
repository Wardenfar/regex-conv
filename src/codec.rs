pub trait Codec {
    const BITS: usize;

    fn name() -> &'static str;
    fn symbols() -> impl Iterator<Item = (u8, u32)>;
}

pub struct Base64;

impl Codec for Base64 {
    const BITS: usize = 6;
    fn name() -> &'static str {
        "Base64"
    }

    fn symbols() -> impl Iterator<Item = (u8, u32)> {
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .iter()
            .enumerate()
            .map(|(idx, char)| (*char, idx as _))
    }
}

pub struct Hex;

impl Codec for Hex {
    const BITS: usize = 4;
    fn name() -> &'static str {
        "Hex"
    }

    fn symbols() -> impl Iterator<Item = (u8, u32)> {
        b"0123456789abcdef"
            .iter()
            .enumerate()
            .map(|(idx, char)| (*char, idx as _))
    }
}

pub struct Bin;

impl Codec for Bin {
    const BITS: usize = 1;
    fn name() -> &'static str {
        "Binary"
    }

    fn symbols() -> impl Iterator<Item = (u8, u32)> {
        b"01"
            .iter()
            .enumerate()
            .map(|(idx, char)| (*char, idx as _))
    }
}

pub struct Raw;

impl Codec for Raw {
    const BITS: usize = 8;
    fn name() -> &'static str {
        "Raw"
    }

    fn symbols() -> impl Iterator<Item = (u8, u32)> {
        (0..=255).map(|x| (x as _, x as _))
    }
}
