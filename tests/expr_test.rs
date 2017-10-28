extern crate nom;
extern crate bbb;

use bbb::expr::*;
use Expr::*;
use nom::IResult::*;
use bbb::numeral::Numeral::*;
use bbb::ops::BinOp::*;
use bbb::ops::UnOp::*;


#[test]
fn time_expr() {
    let exp1 = "t".as_bytes();
    let exp2 = " t".as_bytes();
    let exp3 = "t ".as_bytes();
    assert_eq!(expr(exp1), Done(&b""[..], Time));
    assert_eq!(expr(exp2), Done(&b""[..], Time));
    assert_eq!(expr(exp3), Done(&b""[..], Time));

    let exp1 = "T".as_bytes();
    let exp2 = " T".as_bytes();
    let exp3 = "T ".as_bytes();
    assert_eq!(expr(exp1), Done(&b""[..], Time));
    assert_eq!(expr(exp2), Done(&b""[..], Time));
    assert_eq!(expr(exp3), Done(&b""[..], Time));
}

#[test]
fn number_expr() {
    let exp1 = "10".as_bytes();
    assert_eq!(expr(exp1), Done(&b""[..], Num(Int(10))));

    let exp2 = "10.1".as_bytes();
    assert_eq!(expr(exp2), Done(&b""[..], Num(Float(10.1))));

    let exp3 = "-10".as_bytes();
    assert_eq!(expr(exp3), Done(&b""[..], Num(Int(-10))));

    let exp4 = "-10.1".as_bytes();
    assert_eq!(expr(exp4), Done(&b""[..], Num(Float(-10.1))));

    let exp5 = "0.1".as_bytes();
    assert_eq!(expr(exp5), Done(&b""[..], Num(Float(0.1))));

    let exp6 = "-0.1".as_bytes();
    assert_eq!(expr(exp6), Done(&b""[..], Num(Float(-0.1))));
}

#[test]
fn binop_expr() {
    let exp1 = "1 + -1".as_bytes();
    assert_eq!(
        expr(exp1),
        Done(&b""[..],
             BinExpr(
                 Box::new(Num(Int(1))),
                 Add,
                 Box::new(Num(Int(-1)))
             )
        )
    );

    let exp2 = "(1 + 1)".as_bytes();
    assert_eq!(
        expr(exp2),
        Done(&b""[..],
             BinExpr(
                 Box::new(Num(Int(1))),
                 Add,
                 Box::new(Num(Int(1)))
             )
        )
    );

    let exp3 = "(t) + -10".as_bytes();
    assert_eq!(
        expr(exp3),
        Done(&b""[..],
             BinExpr(
                 Box::new(Time),
                 Add,
                 Box::new(Num(Int(-10)))
             )
        )
    );
}

#[test]
fn unop_expr() {
    let exp1 = "~10".as_bytes();
    assert_eq!(
        expr(exp1),
        Done(&b""[..], UnExpr(BitNot, Box::new(Num(Int(10)))))
    );

    let exp2 = "-(10 * t)".as_bytes();
    assert_eq!(
        expr(exp2),
        Done(
            &b""[..],
            UnExpr(
                Neg,
                Box::new(
                    BinExpr(
                        Box::new(Num(Int(10))),
                        Mul,
                        Box::new(Time)
                    )
                )
            )
        )
    );
}

#[test]
fn expr_precedence() {
    let exp1 = "1 + -2 * 3".as_bytes();
    assert_eq!(
        parse(exp1),
        Ok(
            Box::new(
                BinExpr(
                    Box::new(
                        BinExpr(
                            Box::new(Num(Int(1))),
                            Add,
                            Box::new(Num(Int(-2)))
                        ),
                    ),
                    Mul,
                    Box::new(Num(Int(3)))
                )
            )
        )
    );
}
