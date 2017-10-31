extern crate bbb_core;
extern crate nom;

use bbb_core::parser::*;
use bbb_core::expr::Expr::*;
use bbb_core::numeral::Numeral::*;
use bbb_core::ops::BinOp::*;
use bbb_core::ops::UnOp::*;
use nom::*;
use nom::IResult::*;

#[test]
fn time_parse() {
    let exp1 = "t";
    let exp2 = " t";
    let exp3 = "t ";

    assert_eq!(parse(exp1), Ok(Box::new(Time)));
    assert_eq!(parse(exp2), Ok(Box::new(Time)));
    assert_eq!(parse(exp3), Ok(Box::new(Time)));

    let exp1 = "T";
    let exp2 = " T";
    let exp3 = "T ";
    assert_eq!(parse(exp1), Ok(Box::new(Time)));
    assert_eq!(parse(exp2), Ok(Box::new(Time)));
    assert_eq!(parse(exp3), Ok(Box::new(Time)));
}

#[test]
fn number_parse() {
    let exp1 = "10";
    assert_eq!(parse(exp1), Ok(Box::new(Num(Int(10)))));

    let exp2 = "10.1";
    assert_eq!(parse(exp2), Ok(Box::new(Num(Float(10.1)))));

    let exp3 = "-10";
    assert_eq!(parse(exp3), Ok(Box::new(Num(Int(-10)))));

    let exp4 = "-10.1";
    assert_eq!(parse(exp4), Ok(Box::new(Num(Float(-10.1)))));

    let exp5 = "0.1";
    assert_eq!(parse(exp5), Ok(Box::new(Num(Float(0.1)))));

    let exp6 = "-0.1";
    assert_eq!(parse(exp6), Ok(Box::new(Num(Float(-0.1)))));
}

#[test]
fn binop_parse() {
    let exp1 = "1 + -1";
    assert_eq!(
        parse(exp1),
        Ok(Box::new(
            BinExpr(Box::new(Num(Int(1))), Add, Box::new(Num(Int(-1)))),
        ))
    );

    let exp2 = "(1 + 1)";
    assert_eq!(
        parse(exp2),
        Ok(Box::new(
            BinExpr(Box::new(Num(Int(1))), Add, Box::new(Num(Int(1)))),
        ))
    );

    let exp3 = "(t) + -10";
    assert_eq!(
        parse(exp3),
        Ok(Box::new(
            BinExpr(Box::new(Time), Add, Box::new(Num(Int(-10)))),
        ))
    );

    let exp4 = "1 + 1 - 1";
    assert_eq!(
        parse(exp4),
        Ok(Box::new(BinExpr(
            Box::new(
                BinExpr(Box::new(Num(Int(1))), Add, Box::new(Num(Int(1)))),
            ),
            Sub,
            Box::new(Num(Int(1))),
        )))
    );
}

#[test]
fn unop_parse() {
    let exp1 = "~10";
    assert_eq!(
        parse(exp1),
        Ok(Box::new(UnExpr(BitNot, Box::new(Num(Int(10))))))
    );

    let exp2 = "-(10 * t)";
    assert_eq!(
        parse(exp2),
        Ok(Box::new(UnExpr(
            BitNot,
            Box::new(
                BinExpr(Box::new(Num(Int(10))), Mul, Box::new(Time)),
            ),
        )))
    );
}

#[test]
fn expr_precedence() {
    let exp1 = "1 + -2 * 3";
    assert_eq!(
        parse(exp1),
        Ok(Box::new(BinExpr(
            Box::new(
                BinExpr(Box::new(Num(Int(1))), Add, Box::new(Num(Int(-2)))),
            ),
            Mul,
            Box::new(Num(Int(3))),
        )))
    );
}
