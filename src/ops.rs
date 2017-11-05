#[derive(Clone, Debug, PartialEq)]
pub enum BinOp {
    One(BinOp1),
    Two(BinOp2),
    Three(BitShift),
    Four(BitAnd),
    Five(BitXOr),
    Six(BitOr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp1 {
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinOp2 {
    Sub,
    Add,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BitShift {
    Right,
    Left,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BitAnd;

#[derive(Clone, Debug, PartialEq)]
pub struct BitXOr;

#[derive(Clone, Debug, PartialEq)]
pub struct BitOr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UnOp {
    Neg,
    BoolNot,
    BitNot,
}

named!(pub bit_and<BitAnd>, value!(BitAnd, tag!("&")));
named!(pub bit_or<BitOr>, value!(BitOr, tag!("|")));
named!(pub bit_xor<BitXOr>, value!(BitXOr, tag!("^")));

named!(pub mul_or_div<BinOp1>,
       alt!(
           value!(BinOp1::Div, tag!("/")) |
           value!(BinOp1::Mul, tag!("*"))
       )
);

named!(pub add_or_sub<BinOp2>,
       alt!(
           value!(BinOp2::Sub, tag!("-")) |
           value!(BinOp2::Add, tag!("+"))
       )
);

named!(pub bit_shift<BitShift>,
       alt!(
           value!(BitShift::Right, tag!(">>")) |
           value!(BitShift::Left, tag!("<<"))
       )
);

named!(pub binary_op<BinOp>,
       alt!(
           map!(mul_or_div, BinOp::One) |
           map!(add_or_sub, BinOp::Two) |
           map!(bit_shift, BinOp::Three) |
           map!(bit_and, BinOp::Four) |
           map!(bit_xor, BinOp::Five) |
           map!(bit_or, BinOp::Six)
       )
);

named!(neg_op<UnOp>,
       value!(UnOp::Neg,
              recognize!(
                  tuple!(
                      char!('-'),
                      peek!(alt!(char!('(') | char!('t')))
                  )
              )
       )
);

named!(pub unop<UnOp>,
       alt!(
           neg_op |
           value!(UnOp::BoolNot, char!('!')) |
           value!(UnOp::BitNot, char!('~'))
       )
);
