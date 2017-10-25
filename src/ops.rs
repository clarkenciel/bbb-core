use nom::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BinOp {
    Sub,
    Add,
    Div,
    Mul,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    BoolNot,
    BitNot,
}

named!(pub binop<BinOp>,
    alt!(
        value!(BinOp::Sub, tag!("-")) |
        value!(BinOp::Add, tag!("+")) |
        value!(BinOp::Div, tag!("/")) |
        value!(BinOp::Mul, tag!("*"))
    )
);

named!(pub unop<UnOp>,
       alt!(
           value!(UnOp::Neg, tag!("-")) |
           value!(UnOp::BoolNot, tag!("!")) |
           value!(UnOp::BitNot, tag!("~"))
       )
);
