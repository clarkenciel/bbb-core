extern crate bbb;

use bbb::numeral::*;

#[test]
fn single_digit_int() {
    let string = "1".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Int(1));
}

#[test]
fn multi_digit_int() {
    let string = "100".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Int(100));
}

#[test]
fn negative_int() {
    let string = "-100".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Int(-100));
}

#[test]
fn leading_float() {
    let string = "1.0".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Float(1.0));
}

#[test]
fn leading_zero_float() {
    let string = "0.1".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Float(0.1));
}

#[test]
fn negative_float() {
    let string = "-1.0001".as_bytes();
    assert_eq!(number(string).to_result().unwrap(), Numeral::Float(-1.0001));
}
