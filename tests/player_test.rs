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
    let mut player = player::Player::new(44_100.0, 256).unwrap();
    player.play(signal.clone()).unwrap();
    sleep(Duration::from_millis(5_000));
    player.stop().unwrap();
}
