extern crate bbb_core;

use bbb_core::parser::*;
use bbb_core::expr::Expr::*;
use bbb_core::numeral::Numeral::*;
use bbb_core::ops::*;
use bbb_core::ops::UnOp::*;

#[test]
fn number_parse_test() {
    let e = "1";
    assert_eq!(
        parse(e),
        Ok(Num(Int(1)))
    );

    let e = "-1";
    assert_eq!(
        parse(e),
        Ok(Num(Int(-1)))
    );

    let e = " -10 ";
    assert_eq!(
        parse(e),
        Ok(Num(Int(-10)))
    );

    let e = "(-10)";
    assert_eq!(
        parse(e),
        Ok(Num(Int(-10)))
    );
}

#[test]
fn time_parse_test() {
    let e = "t";
    assert_eq!(
        parse(e),
        Ok(Time)
    );

    let e = " t ";
    assert_eq!(
        parse(e),
        Ok(Time)
    );

    let e = "(t)";
    assert_eq!(
        parse(e),
        Ok(Time)
    );
}

#[test]
fn unary_parse_test() {
    let e = "-(10)";
    assert_eq!(
        parse(e),
        Ok(UnExpr(Neg, Box::new(Num(Int(10)))))
    );

    let e = "~10";
    assert_eq!(
        parse(e),
        Ok(UnExpr(BitNot, Box::new(Num(Int(10)))))
    );

    let e = "!10";
    assert_eq!(
        parse(e),
        Ok(UnExpr(BoolNot, Box::new(Num(Int(10)))))
    );
}

#[test]
fn simple_binary_parse_test() {
    let e = "1 + 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::Two(BinOp2::Add),
                Box::new(Num(Int(1)))
            )
        )
    );

    let e = "1 * 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::One(BinOp1::Mul),
                Box::new(Num(Int(1)))
            )
        )
    );

    let e = "1 >> 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::Three(BitShift::Right),
                Box::new(Num(Int(1)))
            )
        )
    );

    let e = "1 & 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::Four(BitAnd),
                Box::new(Num(Int(1)))
            )
        )
    );

    let e = "1 ^ 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::Five(BitXOr),
                Box::new(Num(Int(1)))
            )
        )
    );

    let e = "1 | 1";
    assert_eq!(
        parse(e),
        Ok(
            BinExpr(
                Box::new(Num(Int(1))),
                BinOp::Six(BitOr),
                Box::new(Num(Int(1)))
            )
        )
    );
}
