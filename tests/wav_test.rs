extern crate bbb_core;

use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::wav;

#[test]
fn wav_file_test() {
    let e = "(t * 9 & t >> 4 | t * 5 & t >> 7 | t * 3 & t / 1024) - 1";
    let mut signal = ExprSignal::from(parse(e).unwrap());
    wav::record("test.wav", 10, &mut signal).unwrap();
}
