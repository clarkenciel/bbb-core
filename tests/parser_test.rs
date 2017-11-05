extern crate bbb_core;
extern crate nom;

use bbb_core::parser::*;
use bbb_core::expr::Expr::*;
use bbb_core::numeral::Numeral::*;
use bbb_core::ops;
use bbb_core::ops::UnOp::*;
use nom::IResult::*;

// #[test]
// fn time_parse() {
//     let exp1 = "t";
//     let exp2 = " t";
//     let exp3 = "t ";

//     assert_eq!(parse(exp1), Ok(Box::new(Time)));
//     assert_eq!(parse(exp2), Ok(Box::new(Time)));
//     assert_eq!(parse(exp3), Ok(Box::new(Time)));

//     let exp1 = "T";
//     let exp2 = " T";
//     let exp3 = "T ";
//     assert_eq!(parse(exp1), Ok(Box::new(Time)));
//     assert_eq!(parse(exp2), Ok(Box::new(Time)));
//     assert_eq!(parse(exp3), Ok(Box::new(Time)));
// }

// #[test]
// fn number_parse() {
//     let exp1 = "10";
//     assert_eq!(parse(exp1), Ok(Box::new(Num(Int(10)))));

//     let exp2 = "10.1";
//     assert_eq!(parse(exp2), Ok(Box::new(Num(Float(10.1)))));

//     let exp3 = "-10";
//     assert_eq!(parse(exp3), Ok(Box::new(Num(Int(-10)))));

//     let exp4 = "-10.1";
//     assert_eq!(parse(exp4), Ok(Box::new(Num(Float(-10.1)))));

//     let exp5 = "0.1";
//     assert_eq!(parse(exp5), Ok(Box::new(Num(Float(0.1)))));

//     let exp6 = "-0.1";
//     assert_eq!(parse(exp6), Ok(Box::new(Num(Float(-0.1)))));
// }

#[test]
fn empty_exp1_test() {
    let e = "".as_bytes();
    assert_eq!(
        exp1(e),
        Done(&b""[..],
             Exp1::Empty)
    );
}

#[test]
fn seq_exp1_test() {
    let e = "+ 1".as_bytes();
    assert_eq!(
        exp1(e),
        Done(&b""[..],
             Exp1::Seq(
                 ops::BinOp2::Add,
                 Box::new(Exp(
                     Box::new(Term(
                         Box::new(Factor::Expr(Box::new(Num(Int(1))))),
                         Box::new(Term1::Empty)
                     )),
                     Box::new(Exp1::Empty))
                 )
             ))
    );
}

#[test]
fn number_factor_test() {
    let e = "1".as_bytes();
    assert_eq!(
        factor(e),
        Done(&b""[..],
             Factor::Expr(Box::new(Num(Int(1))))
        )
    );

    let e = "-1".as_bytes();
    assert_eq!(
        factor(e),
        Done(&b""[..],
             Factor::Expr(Box::new(Num(Int(-1))))
        )
    );
}

#[test]
fn time_factor_test() {
    let e = "t".as_bytes();
    assert_eq!(
        factor(e),
        Done(&b""[..],
             Factor::Expr(Box::new(Time)))
    );
}

#[test]
fn unary_factor_test() {
    let e = "-t".as_bytes();
    assert_eq!(
        factor(e),
        Done(&b""[..],
             Factor::Unary(Neg, Box::new(Factor::Expr(Box::new(Time)))))
    );

    let e = "-(1)".as_bytes();
    assert_eq!(
        factor(e),
        Done(&b""[..],
             Factor::Unary(
                 Neg,
                 Box::new(Factor::Exp(
                     Box::new(Exp(
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         )),
                         Box::new(Exp1::Empty)
                     ))
                 ))
             )
        )
    );
}

#[test]
fn empty_term1_test() {
    let e = "".as_bytes();
    assert_eq!(
        term1(e),
        Done(&b""[..], Term1::Empty)
    );
}

#[test]
fn seq_term1_test() {
    let e = "* 10".as_bytes();
    assert_eq!(
        term1(e),
        Done(&b""[..],
             Term1::Seq(
                 ops::BinOp1::Mul,
                 Box::new(Term(
                     Box::new(Factor::Expr(
                         Box::new(Num(Int(10)))
                     )),
                     Box::new(Term1::Empty)
                 ))
             )
        )
    );
}

#[test]
fn number_exp_test() {
    let e = "1".as_bytes();
    assert_eq!(
        exp(e),
        Done(&b""[..],
             Exp(Box::new(Term(Box::new(Factor::Expr(Box::new(Num(Int(1))))),
                               Box::new(Term1::Empty))),
                 Box::new(Exp1::Empty)
             )
        )
    );
}

#[test]
fn add_exp_test() {
    let e = "1 + 1".as_bytes();

    assert_eq!(
        exp(e),
        Done(&b""[..],
             Exp(
                 Box::new(Term(
                     Box::new(Factor::Expr(
                         Box::new(Num(Int(1)))
                     )),
                     Box::new(Term1::Empty)
                 )),
                 Box::new(Exp1::Seq(
                     ops::BinOp2::Add,
                     Box::new(Exp(
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         )),
                         Box::new(Exp1::Empty)
                     ))
                 ))
             )
        )
    );
}

#[test]
fn mul_exp_test() {
    let e = "1 * 1".as_bytes();

    assert_eq!(
        exp(e),
        Done(&b""[..],
             Exp(
                 Box::new(Term(
                     Box::new(Factor::Expr(
                         Box::new(Num(Int(1)))
                     )),
                     Box::new(Term1::Seq(
                         ops::BinOp1::Mul,
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         ))
                     ))
                 )),
                 Box::new(Exp1::Empty)
             )
        )
    );
}

#[test]
fn three_term_test() {
    let e = "1 + 1 + 1".as_bytes();

    assert_eq!(
        exp(e),
        Done(&b""[..],
             Exp(
                 Box::new(Term(
                     Box::new(Factor::Expr(
                         Box::new(Num(Int(1)))
                     )),
                     Box::new(Term1::Empty)
                 )),
                 Box::new(Exp1::Seq(
                     ops::BinOp2::Add,
                     Box::new(Exp(
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         )),
                         Box::new(Exp1::Seq(
                             ops::BinOp2::Add,
                             Box::new(Exp(
                                 Box::new(Term(
                                     Box::new(Factor::Expr(
                                         Box::new(Num(Int(1)))
                                     )),
                                     Box::new(Term1::Empty)
                                 )),
                                 Box::new(Exp1::Empty)
                             ))
                         ))
                     ))
                 ))
             )
        )
    );
}

#[test]
fn mixed_exp_test() {
    let e = "1 * 1 + 1".as_bytes();

    assert_eq!(
        exp(e),
        Done(&b""[..],
             Exp(
                 Box::new(Term(
                     Box::new(Factor::Expr(
                         Box::new(Num(Int(1)))
                     )),
                     Box::new(Term1::Seq(
                         ops::BinOp1::Mul,
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         ))
                     ))
                 )),
                 Box::new(Exp1::Seq(
                     ops::BinOp2::Add,
                     Box::new(Exp(
                         Box::new(Term(
                             Box::new(Factor::Expr(
                                 Box::new(Num(Int(1)))
                             )),
                             Box::new(Term1::Empty)
                         )),
                         Box::new(Exp1::Empty)
                     ))
                 ))
             )
        )
    );
}
