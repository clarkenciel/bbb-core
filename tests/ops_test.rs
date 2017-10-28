extern crate nom;
extern crate bbb;

use bbb::ops::*;
use nom::Needed::Size;
use nom::IResult::*;

#[test]
fn unary_parse() {
    assert_eq!(
        unop("-(".as_bytes()),
        Done(&b"("[..], UnOp::Neg)
    );

    assert_eq!(
        unop("-".as_bytes()),
        Incomplete(Size(2))
    );

    assert_eq!(
        unop("~".as_bytes()),
        Done(&b""[..], UnOp::BitNot)
    );

    assert_eq!(
        unop("!".as_bytes()),
        Done(&b""[..], UnOp::BoolNot)
    );
}

#[test]
fn binary_parse() {
    assert_eq!(
        add_or_sub("-".as_bytes()).to_result().unwrap(),
        BinOp::Sub
    );

    assert_eq!(
        add_or_sub("+".as_bytes()).to_result().unwrap(),
        BinOp::Add
    );

    assert_eq!(
        mul_or_div("/".as_bytes()).to_result().unwrap(),
        BinOp::Div
    );

    assert_eq!(
        mul_or_div("*".as_bytes()).to_result().unwrap(),
        BinOp::Mul
    );

    assert_eq!(
        bit_shift(">>".as_bytes()).to_result().unwrap(),
        BinOp::BitShiftR
    );

    assert_eq!(
        bit_shift("<<".as_bytes()).to_result().unwrap(),
        BinOp::BitShiftL
    );

    assert_eq!(
        bit_and("&".as_bytes()).to_result().unwrap(),
        BinOp::BitAnd
    );

    assert_eq!(
        bit_or("|".as_bytes()).to_result().unwrap(),
        BinOp::BitOr
    );

    assert_eq!(
        bit_xor("^".as_bytes()).to_result().unwrap(),
        BinOp::BitXOr
    );
}

#[test]
fn binary_precedence() {
    use BinOp::*;

    assert_eq!(Div < Mul, false);
    assert_eq!(Add < Mul, true);
    assert_eq!(Sub < Mul, true);
    assert_eq!(Add < Div, true);
    assert_eq!(Sub < Div, true);
    assert_eq!(Sub < Add, false);
    assert_eq!(BitShiftR < Add, true);
    assert_eq!(BitShiftR < Sub, true);
    assert_eq!(BitShiftL < Add, true);
    assert_eq!(BitShiftL < Sub, true);
    assert_eq!(BitShiftR < BitShiftL, false);
    assert_eq!(BitAnd < BitShiftR, true);
    assert_eq!(BitAnd < BitShiftL, true);
    assert_eq!(BitXOr < BitAnd, true);
    assert_eq!(BitOr < BitXOr, true);
}
