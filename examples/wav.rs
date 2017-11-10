extern crate bbb_core;

use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::wav;

fn main() {
    let e = "(t * 9 & t >> 4 | t * 5 & t >> 7 | t * 3 & t / 1024) - 1";
    let mut signal = ExprSignal::from(parse(e).unwrap());

    println!("writing the following equation to ./test.wav: {}", e);
    wav::Recorder::new(44_100).record("test.wav", 60.0, &mut signal).unwrap();
}
