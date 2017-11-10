extern crate bbb_core;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread::sleep;

use bbb_core::parser::parse;
use bbb_core::signal::ExprSignal;
use bbb_core::player;

fn main() {
    let e = "((t<<1)^((t<<1)+(t>>7)&t>>12))|t>>(4-(1^7&(t>>19)))|t>>7";
    let signal = Arc::new(Mutex::new(ExprSignal::from(parse(e).unwrap())));
    let mut player = player::Player::new(8_000.0, 1024).unwrap();

    println!("playing equation: {}", e);
    player.play(signal).unwrap();
    sleep(Duration::from_secs(60));
    player.stop().unwrap();
}
