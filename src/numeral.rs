use nom::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Numeral {
    Float(f32),
    Int(i32),
}

// named!(pub number<Numeral>,
//        alt!(
//            map_result!(
//                do_parse!(prefix: digit >> dot: tag!(".") >> postfix: digit >> (prefix, dot, postfix)),
//                |(pre, d, post)|
