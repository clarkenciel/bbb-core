use ops::{BinOp, UnOp};
use numeral::Numeral;

pub enum Expr {
    Time,
    Num(Numeral),
    UnExpr(UnOp, Box<Expr>),
    BinExpr(Box<Expr>, BinOp, Box<Expr>),
}
