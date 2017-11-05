extern crate nom;
extern crate bbb_core;

use bbb_core::ops::*;
use nom::Needed::Size;
use nom::IResult::*;

#[test]
fn unary_parse() {
    assert_eq!(
        unop("-(".as_bytes()),
        Done(&b"("[..], UnOp::Neg)
    );

    assert_eq!(
        unop("-".as_bytes()),
        Incomplete(Size(2))
    );

    assert_eq!(
        unop("~".as_bytes()),
        Done(&b""[..], UnOp::BitNot)
    );

    assert_eq!(
        unop("!".as_bytes()),
        Done(&b""[..], UnOp::BoolNot)
    );
}
