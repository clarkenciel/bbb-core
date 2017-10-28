use nom::*;
use std::str::{self, FromStr};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Numeral {
    Float(f32),
    Int(i32),
}

impl From<i32> for Numeral {
    fn from(num: i32) -> Numeral {
        Numeral::Int(num)
    }
}

impl From<f32> for Numeral {
    fn from(num: f32) -> Numeral {
        Numeral::Float(num)
    }
}

named!(float<f32>,
       map_res!(
           map_res!(
               recognize!(
                   tuple!(
                       opt!(tag!("-")),
                       digit,
                       complete!(preceded!(tag!("."), opt!(digit)))
                   )
               ),
               str::from_utf8
           ),
           f32::from_str
       )
);

named!(int<i32>,
       map_res!(
           map_res!(
               recognize!(
                   tuple!(opt!(tag!("-")), digit)
               ),
               str::from_utf8
           ),
           |s| i32::from_str_radix(s, 10)
       )
);

named!(pub number<Numeral>,
       alt!(map!(float, Numeral::from) | map!(int, Numeral::from))
);
