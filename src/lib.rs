extern crate hound;
#[macro_use]
extern crate nom;
extern crate sample;

const SAMPLE_RATE: u32 = 8_000;

pub mod expr;
pub mod eval;
pub mod numeral;
pub mod ops;
pub mod parser;
pub mod signal;
pub mod wav;
