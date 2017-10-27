use ops::{BinOp, UnOp};
use numeral::*;
use ops::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Time,
    Num(Numeral),
    UnExpr(UnOp, Box<Expr>),
    BinExpr(Box<Expr>, BinOp, Box<Expr>),
}

named!(time<Expr>, value!(Expr::Time, alt!(tag!("t") | tag!("T"))));
named!(expr_num<Expr>, map!(number, Expr::Num));
named!(expr_unop<Expr>,
       map!(
           tuple!(unop, expr),
           |(op, expr)| Expr::UnExpr(op, Box::new(expr))
       )
);

named!(expr_binop<Expr>,
       map!(
           tuple!(expr, binop, expr),
           |(expr1, op, expr2)| Expr::BinExpr(Box::new(expr1), op, Box::new(expr2))
       )
);

named!(pub expr<Expr>, alt!(expr_binop | expr_unop | expr_num | time));
