
use numeral::*;
use ops::*;
use expr::Expr;
use self::Expr::*;

#[derive(Debug, PartialEq)]
pub struct Exp(pub Box<Term>, pub Box<Exp1>);

#[derive(Debug, PartialEq)]
pub enum Exp1 {
    Empty,
    Seq(BinOp2, Box<Exp>),
}

#[derive(Debug, PartialEq)]
pub struct Term(pub Box<Factor>, pub Box<Term1>);

#[derive(Debug, PartialEq)]
pub enum Term1 {
    Empty,
    Seq(BinOp1, Box<Term>),
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Expr(Box<Expr>),
    Exp(Box<Exp>),
    Unary(UnOp, Box<Factor>),
}

impl From<Factor> for Expr {
    fn from(f: Factor) -> Self {
        match f {
            Factor::Expr(expr) => *expr,
            Factor::Exp(exp) => Expr::from(*exp),
            Factor::Unary(op, sub_f) => {
                UnExpr(
                    op,
                    Box::new(Expr::from(*sub_f)),
                )
            }
        }
    }
}

impl From<Term> for Expr {
    fn from(t: Term) -> Self {
        match *t.1 {
            Term1::Empty => Expr::from(*t.0),
            Term1::Seq(op, t2) => BinExpr(
                Box::new(Expr::from(*t.0)),
                BinOp::One(op),
                Box::new(Expr::from(*t2))
            )
        }
    }
}

impl From<Exp> for Expr {
    fn from(e: Exp) -> Self {
        match *e.1 {
            Exp1::Empty => Expr::from(*e.0),
            Exp1::Seq(op, e2) => BinExpr(
                Box::new(Expr::from(*e.0)),
                BinOp::Two(op),
                Box::new(Expr::from(*e2))
            ),
        }
    }
}

pub fn parse(input: &str) -> Result<Expr, String> {
    exp(input.as_bytes())
        .to_result()
        .map(Expr::from)
        .map_err(|e| format!("{}", e))
}

named!(time<Expr>, value!(Time, alt!(char!('T') | char!('t'))));
named!(num<Expr>, map!(number, Num));

named!(pub exp<Exp>, ws!(
    map!(pair!(term, exp1), |(t, e)| Exp(Box::new(t), Box::new(e)))
));

named!(pub exp1<Exp1>, ws!(alt!(
    value!(Exp1::Empty, empty) |
    map!(pair!(add_or_sub, exp), |(o, e)| Exp1::Seq(o, Box::new(e))) |
    value!(Exp1::Empty, tag!(""))
)));

named!(pub term<Term>, ws!(
    map!(pair!(factor, term1), |(f, t)| Term(Box::new(f), Box::new(t)))
));

named!(empty<&[u8]>, alt!(eof!()));

named!(pub term1<Term1>, ws!(alt!(
    value!(Term1::Empty, empty) |
    map!(pair!(mul_or_div, term), |(o, t)| Term1::Seq(o, Box::new(t))) |
    value!(Term1::Empty, tag!(""))
)));

named!(pub factor<Factor>, ws!(
    alt!(
        map!(
            alt!(time | num),
            |l| Factor::Expr(Box::new(l))
        ) |
        map!(
            delimited!(char!('('), exp, char!(')')),
            |e| Factor::Exp(Box::new(e))
        ) |
        map!(
            pair!(unop, factor),
            |(o, f)| Factor::Unary(o, Box::new(f))
        )
    )
));
