extern crate bbb_core;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;

use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::player;

#[test]
fn player_test() {
    let e = "(t * 9 & t >> 4 | t * 5 & t >> 7 | t * 3 & t / 1024) - 1";
    let signal = Arc::new(Mutex::new(ExprSignal::from(parse(e).unwrap())));
    player::Player::new(44_100).unwrap().play(signal);
    sleep(Duration::from_millis(1_000));
}
