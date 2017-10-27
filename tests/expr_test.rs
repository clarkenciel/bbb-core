extern crate bbb;

use bbb::expr::*;
use bbb::ops::*;
use bbb::numeral::*;

#[test]
fn single_expression_no_paren() {
    let expression = "t & 96".as_bytes();
    let expected = Expr::BinExpr(
        Box::new(Expr::Time),
        BinOp::BitAnd,
        Box::new(Expr::Num(Numeral::Int(96)))
    );
    assert_eq!(expr(expression).to_result().unwrap(), expected);
}

#[test]
fn real_expression() {
    let expression = "((t<<1) ^ ((t<<1) + (t>>7) & t>>12)) | t >> (4-(1^7&(t>>19))) | t >> 7";
}
