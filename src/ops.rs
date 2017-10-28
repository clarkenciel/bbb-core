use std::cmp::Ordering::{Less, Equal, Greater};
use std::cmp::Ordering;

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    Sub,
    Add,
    Div,
    Mul,
    BitShiftR,
    BitShiftL,
    BitAnd,
    BitXOr,
    BitOr,
}

impl PartialOrd for BinOp {
    fn partial_cmp(&self, other: &BinOp) -> Option<Ordering> {
        match (self, other) {
            (&BinOp::Mul, &BinOp::Div) => Some(Equal),
            (&BinOp::Mul, &BinOp::Mul) => Some(Equal),
            (&BinOp::Mul, _) => Some(Greater),
            (&BinOp::Div, &BinOp::Mul) => Some(Equal),
            (&BinOp::Div, &BinOp::Div) => Some(Equal),
            (&BinOp::Div, _) => Some(Greater),
            (_, &BinOp::Div) => Some(Less),
            (_, &BinOp::Mul) => Some(Less),

            (&BinOp::Add, &BinOp::Add) => Some(Equal),
            (&BinOp::Add, &BinOp::Sub) => Some(Equal),
            (&BinOp::Sub, &BinOp::Add) => Some(Equal),
            (&BinOp::Sub, &BinOp::Sub) => Some(Equal),
            (&BinOp::Sub, _) => Some(Greater),
            (&BinOp::Add, _) => Some(Greater),
            (_, &BinOp::Sub) => Some(Less),
            (_, &BinOp::Add) => Some(Less),

            (&BinOp::BitShiftR, &BinOp::BitShiftL) => Some(Equal),
            (&BinOp::BitShiftL, &BinOp::BitShiftR) => Some(Equal),
            (&BinOp::BitShiftR, _) => Some(Greater),
            (&BinOp::BitShiftL, _) => Some(Greater),
            (_, &BinOp::BitShiftR) => Some(Less),
            (_, &BinOp::BitShiftL) => Some(Less),

            (&BinOp::BitAnd, _) => Some(Greater),
            (_, &BinOp::BitAnd) => Some(Less),

            (&BinOp::BitXOr, _) => Some(Greater),
            (&BinOp::BitOr, _) => Some(Less),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    BoolNot,
    BitNot,
}

named!(pub bit_and<BinOp>, value!(BinOp::BitAnd, tag!("&")));
named!(pub bit_or<BinOp>, value!(BinOp::BitOr, tag!("|")));
named!(pub bit_xor<BinOp>, value!(BinOp::BitXOr, tag!("^")));

named!(pub mul_or_div<BinOp>,
       alt!(
           value!(BinOp::Div, tag!("/")) |
           value!(BinOp::Mul, tag!("*"))
       )
);

named!(pub add_or_sub<BinOp>,
       alt!(
           value!(BinOp::Sub, tag!("-")) |
           value!(BinOp::Add, tag!("+"))
       )
);

named!(pub bit_shift<BinOp>,
       alt!(
           value!(BinOp::BitShiftR, tag!(">>")) |
           value!(BinOp::BitShiftL, tag!("<<"))
       )
);

named!(neg_op<UnOp>,
       value!(UnOp::Neg,
              recognize!(
                  tuple!(
                      tag!("-"),
                      peek!(tag!("("))
                  )
              )
       )
);

named!(pub unop<UnOp>,
       alt!(
           neg_op |
           value!(UnOp::BoolNot, tag!("!")) |
           value!(UnOp::BitNot, tag!("~"))
       )
);
