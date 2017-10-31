use expr::Expr;
use numeral::*;
use ops::*;
use self::Expr::*;

struct Exp(Box<Term>, Box<Exp1>);
enum Exp1 {
    Empty,
    Seq(BinOp, Box<Exp>),
}

struct Term(Box<Factor>, Box<Term1>);
enum Term1 {
    Empty,
    Seq(BinOp, Box<Term>),
}

enum Factor {
    Expr(Box<Expr>),
    Exp(Box<Exp>),
    Unary(UnOp, Box<Factor>),
}

pub fn parse(input: &str) -> Result<Box<Expr>, String> {
    exp(input.as_bytes())
        .to_result()
        .map(|result| extract_expression(&result))
        .map_err(|e| format!("{}", e))
}

fn extract_expression(exp: &Exp) -> Box<Expr> {
    match exp {
        &Exp(ref term, ref expr) => {
            match expr.as_ref() {
                &Exp1::Empty => extract_term(term.as_ref()),
                &Exp1::Seq(ref op, ref exp) => {
                    Box::new(BinExpr(
                        extract_term(term.as_ref()),
                        op.clone(),
                        extract_expression(exp.as_ref()),
                    ))
                }
            }
        }
    }
}

fn extract_term(term: &Term) -> Box<Expr> {
    match term {
        &Term(ref factor, ref term1) => {
            match term1.as_ref() {
                &Term1::Empty => extract_factor(factor.as_ref()),
                &Term1::Seq(ref op, ref term2) => {
                    Box::new(BinExpr(
                        extract_factor(factor.as_ref()),
                        op.clone(),
                        extract_term(term2.as_ref()),
                    ))
                }
            }
        }
    }
}

fn extract_factor(factor: &Factor) -> Box<Expr> {
    match factor {
        &Factor::Expr(ref e) => e.clone(),
        &Factor::Exp(ref e) => extract_expression(e.as_ref()),
        &Factor::Unary(ref op, ref f) => Box::new(UnExpr(op.clone(), extract_factor(f.as_ref()))),
    }
}

named!(time<Expr>, value!(Time, char!('t')));
named!(num<Expr>, map!(number, Num));

named!(exp<Exp>,
       complete!(map!(tuple!(term, exp1), |(t, e)| Exp(Box::new(t), Box::new(e)))));
named!(exp1<Exp1>, alt!(
    map!(tuple!(add_or_sub, exp), |(o, e)| Exp1::Seq(o, Box::new(e))) |
    value!(Exp1::Empty, eof!())
));

named!(term<Term>, map!(tuple!(factor, term1), |(f, t)| Term(Box::new(f), Box::new(t))));

named!(term1<Term1>, alt!(
    map!(tuple!(mul_or_div, term), |(o, t)| Term1::Seq(o, Box::new(t))) |
    value!(Term1::Empty, eof!())
));

named!(factor<Factor>,
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
               tuple!(unop, factor),
               |(o, f)| Factor::Unary(o, Box::new(f))
           )
       )
);
