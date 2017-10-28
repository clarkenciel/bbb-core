use nom::IResult::*;
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

use self::Expr::*;

named!(time<Expr>, value!(Time, alt!(tag!("t") | tag!("T"))));
named!(expr_num<Expr>, map!(number, Num));
named!(parens<Expr>, ws!(delimited!(tag!("("), expr, tag!(")"))));

named!(factor<Expr>,
       ws!(
           alt!(
               alt!(time | expr_num) |
               parens
           )
       )
);

named!(binexp1<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(mul_or_div), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(binexp2<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(add_or_sub), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(binexp3<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(bit_shift), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(binexp4<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(bit_and), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(binexp5<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(bit_xor), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(binexp6<Expr>,
       do_parse!(
           term1: factor >>
           res: pair!(ws!(bit_or), factor) >>
           (BinExpr(Box::new(term1), res.0, Box::new(res.1)))
       )
);

named!(bin_expr<Expr>,
       alt!(binexp1 | binexp2 | binexp3 | binexp4 | binexp5 | binexp6)
);

named!(un_expr<Expr>,
       map!(
           tuple!(unop, expr),
           |(op, expr)| UnExpr(op, Box::new(expr))
       )
);

named!(pub expr<Expr>,
       ws!(alt_complete!(ws!(bin_expr) | ws!(un_expr) | ws!(factor)))
);

named!(remainder<(BinOp, Expr)>,
       complete!(
           pair!(
               ws!(
                   alt!(
                       mul_or_div |
                       add_or_sub |
                       bit_shift |
                       bit_and |
                       bit_xor |
                       bit_or
                   )
               ),
               factor
           )
       )
);

pub fn parse(input: &[u8]) -> Result<Box<Expr>, String> {
    match expr(input) {
        Incomplete(_) => return Err("Incomplete Parse".to_owned()),
        Error(err) => return Err(format!("{}", err)),
        Done(remaining, tree) => {
            if remaining == &b""[..] {
                Ok(Box::new(tree))
            } else {
                parse_remaining(Box::new(tree), remaining)
            }
        }
    }
}

fn parse_remaining(left_tree: Box<Expr>, remaining: &[u8]) -> Result<Box<Expr>, String> {
    let mut running = remaining;
    let mut left = left_tree;

    loop {
        match remainder(running) {
            Incomplete(_) => return Err("Incomplete Parse".to_owned()),
            Error(err) => return Err(format!("{}", err)),
            Done(bytes, (op, right_tree)) => {
                if bytes == &b""[..] {
                    left = Box::new(BinExpr(left, op, Box::new(right_tree)));
                    break;
                } else {
                    running = bytes;
                    left = Box::new(BinExpr(left, op, Box::new(right_tree)));
                }
            }
        }
    }

    Ok(left)
}
