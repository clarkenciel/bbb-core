extern crate bbb;

use bbb::ops::*;

#[test]
fn unary_parse() {
    assert_eq!(
        unop("-".as_bytes()).to_result().unwrap(),
        UnOp::Neg
    );

    assert_eq!(
        unop("~".as_bytes()).to_result().unwrap(),
        UnOp::BitNot
    );

    assert_eq!(
        unop("!".as_bytes()).to_result().unwrap(),
        UnOp::BoolNot
    );
}

#[test]
fn binary_parse() {

}
