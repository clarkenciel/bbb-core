use numeral::*;
use ops::*;


#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Time,
    Num(Numeral),
    UnExpr(UnOp, Box<Expr>),
    BinExpr(Box<Expr>, BinOp, Box<Expr>),
}
