extern crate bbb_core;

use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::wav;

#[test]
fn wav_file_test() {
    let e = "(t * 9 & t >> 4 | t * 5 & t >> 7 | t * 3 & t / 1024) - 1";
    // let e = "((t<<1)^((t<<1)+(t>>7)&t>>12))|t>>(4-(1^7&(t>>19)))|t>>7";
    let mut signal = ExprSignal::from(parse(e).unwrap());
    wav::record("test.wav", 60, &mut signal).unwrap();
}
