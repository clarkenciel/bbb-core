
use numeral::*;
use ops::*;
use expr::Expr;
use self::Expr::*;

pub fn parse(input: &str) -> Result<Expr, String> {
    bit_or_exp(input.as_bytes()).to_result().map(Expr::from).map_err(
        |e| {
            format!("{}", e)
        },
    )
}

#[derive(Debug, PartialEq)]
struct ShiftExp(Box<Exp>, Box<ShiftExp1>);

#[derive(Debug, PartialEq)]
enum ShiftExp1 {
    Empty,
    Seq(BitShift, Box<ShiftExp>),
}

#[derive(Debug, PartialEq)]
struct BitAndExp(Box<ShiftExp>, Box<BitAndExp1>);

#[derive(Debug, PartialEq)]
enum BitAndExp1 {
    Empty,
    Seq(BitAnd, Box<BitAndExp>),
}

#[derive(Debug, PartialEq)]
struct BitXOrExp(Box<BitAndExp>, Box<BitXOrExp1>);

#[derive(Debug, PartialEq)]
enum BitXOrExp1 {
    Empty,
    Seq(BitXOr, Box<BitXOrExp>),
}

#[derive(Debug, PartialEq)]
struct BitOrExp(Box<BitXOrExp>, Box<BitOrExp1>);

#[derive(Debug, PartialEq)]
enum BitOrExp1 {
    Empty,
    Seq(BitOr, Box<BitOrExp>),
}

#[derive(Debug, PartialEq)]
struct Exp(Box<Term>, Box<Exp1>);

#[derive(Debug, PartialEq)]
enum Exp1 {
    Empty,
    Seq(BinOp2, Box<Exp>),
}

#[derive(Debug, PartialEq)]
struct Term(Box<Factor>, Box<Term1>);

#[derive(Debug, PartialEq)]
enum Term1 {
    Empty,
    Seq(BinOp1, Box<Term>),
}

#[derive(Debug, PartialEq)]
enum Factor {
    Expr(Box<Expr>),
    Exp(Box<BitOrExp>),
    Unary(UnOp, Box<Factor>),
}

impl From<Factor> for Expr {
    fn from(f: Factor) -> Self {
        match f {
            Factor::Expr(expr) => *expr,
            Factor::Exp(exp) => Expr::from(*exp),
            Factor::Unary(op, sub_f) => UnExpr(op, Box::new(Expr::from(*sub_f))),
        }
    }
}

impl From<Term> for Expr {
    fn from(t: Term) -> Self {
        match *t.1 {
            Term1::Empty => Expr::from(*t.0),
            Term1::Seq(op, t2) => {
                BinExpr(
                    Box::new(Expr::from(*t.0)),
                    BinOp::One(op),
                    Box::new(Expr::from(*t2)),
                )
            }
        }
    }
}

impl From<Exp> for Expr {
    fn from(e: Exp) -> Self {
        match *e.1 {
            Exp1::Empty => Expr::from(*e.0),
            Exp1::Seq(op, e2) => {
                BinExpr(
                    Box::new(Expr::from(*e.0)),
                    BinOp::Two(op),
                    Box::new(Expr::from(*e2)),
                )
            }
        }
    }
}

impl From<ShiftExp> for Expr {
    fn from(e: ShiftExp) -> Self {
        match *e.1 {
            ShiftExp1::Empty => Expr::from(*e.0),
            ShiftExp1::Seq(op, e2) => {
                BinExpr(
                    Box::new(Expr::from(*e.0)),
                    BinOp::Three(op),
                    Box::new(Expr::from(*e2)),
                )
            }
        }
    }
}

impl From<BitAndExp> for Expr {
    fn from(e: BitAndExp) -> Self {
        match *e.1 {
            BitAndExp1::Empty => Expr::from(*e.0),
            BitAndExp1::Seq(op, e2) => {
                BinExpr(
                    Box::new(Expr::from(*e.0)),
                    BinOp::Four(op),
                    Box::new(Expr::from(*e2)),
                )
            }
        }
    }
}

impl From<BitXOrExp> for Expr {
    fn from(e: BitXOrExp) -> Self {
        match *e.1 {
            BitXOrExp1::Empty => Expr::from(*e.0),
            BitXOrExp1::Seq(op, e2) => {
                BinExpr(
                    Box::new(Expr::from(*e.0)),
                    BinOp::Five(op),
                    Box::new(Expr::from(*e2)),
                )
            }
        }
    }
}

impl From<BitOrExp> for Expr {
    fn from(e: BitOrExp) -> Self {
        match *e.1 {
            BitOrExp1::Empty => Expr::from(*e.0),
            BitOrExp1::Seq(op, e2) => {
                BinExpr(
                    Box::new(Expr::from(*e.0)),
                    BinOp::Six(op),
                    Box::new(Expr::from(*e2)),
                )
            }
        }
    }
}

named!(time<Expr>, value!(Time, alt!(char!('T') | char!('t'))));
named!(num<Expr>, map!(number, Num));

named!(bit_or_exp<BitOrExp>, ws!(
    map!(pair!(bit_xor_exp, bit_or_exp1), |(e, se)| BitOrExp(Box::new(e), Box::new(se)))
));

named!(bit_or_exp1<BitOrExp1>, ws!(alt!(
    value!(BitOrExp1::Empty, empty) |
    map!(pair!(bit_or, bit_or_exp), |(o, e)| BitOrExp1::Seq(o, Box::new(e))) |
    value!(BitOrExp1::Empty, tag!(""))
)));

named!(bit_xor_exp<BitXOrExp>, ws!(
    map!(pair!(bit_and_exp, bit_xor_exp1), |(e, se)| BitXOrExp(Box::new(e), Box::new(se)))
));

named!(bit_xor_exp1<BitXOrExp1>, ws!(alt!(
    value!(BitXOrExp1::Empty, empty) |
    map!(pair!(bit_xor, bit_xor_exp), |(o, e)| BitXOrExp1::Seq(o, Box::new(e))) |
    value!(BitXOrExp1::Empty, tag!(""))
)));

named!(bit_and_exp<BitAndExp>, ws!(
    map!(pair!(shift_exp, bit_and_exp1), |(e, se)| BitAndExp(Box::new(e), Box::new(se)))
));

named!(bit_and_exp1<BitAndExp1>, ws!(alt!(
    value!(BitAndExp1::Empty, empty) |
    map!(pair!(bit_and, bit_and_exp), |(o, e)| BitAndExp1::Seq(o, Box::new(e))) |
    value!(BitAndExp1::Empty, tag!(""))
)));

named!(shift_exp<ShiftExp>, ws!(
    map!(pair!(exp, shift_exp1), |(e, se)| ShiftExp(Box::new(e), Box::new(se)))
));

named!(shift_exp1<ShiftExp1>, ws!(alt!(
    value!(ShiftExp1::Empty, empty) |
    map!(pair!(bit_shift, shift_exp), |(o, e)| ShiftExp1::Seq(o, Box::new(e))) |
    value!(ShiftExp1::Empty, tag!(""))
)));

named!(exp<Exp>, ws!(
    map!(pair!(term, exp1), |(t, e)| Exp(Box::new(t), Box::new(e)))
));

named!(exp1<Exp1>, ws!(alt!(
    value!(Exp1::Empty, empty) |
    map!(pair!(add_or_sub, exp), |(o, e)| Exp1::Seq(o, Box::new(e))) |
    value!(Exp1::Empty, tag!(""))
)));

named!(term<Term>, ws!(
    map!(pair!(factor, term1), |(f, t)| Term(Box::new(f), Box::new(t)))
));

named!(empty<&[u8]>, alt!(eof!()));

named!(term1<Term1>, ws!(alt!(
    value!(Term1::Empty, empty) |
    map!(pair!(mul_or_div, term), |(o, t)| Term1::Seq(o, Box::new(t))) |
    value!(Term1::Empty, tag!(""))
)));

named!(factor<Factor>, ws!(
    alt!(
        map!(
            alt!(time | num),
            |l| Factor::Expr(Box::new(l))
        ) |
        map!(
            delimited!(char!('('), bit_or_exp, char!(')')),
            |e| Factor::Exp(Box::new(e))
        ) |
        map!(
            pair!(unop, factor),
            |(o, f)| Factor::Unary(o, Box::new(f))
        )
    )
));
