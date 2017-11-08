use expr::Expr;
use numeral::Numeral::*;
use ops::*;
use self::Expr::*;
use self::UnOp::*;
use self::BinOp::*;

pub fn eval(time: i32, expression: &Expr) -> Result<i32, &'static str> {
    match expression {
        &Time => Ok(time),
        &Num(Int(i)) => Ok(i),
        &Num(Float(f)) => Ok(f as i32),
        &UnExpr(ref op, ref expr) => eval(time, expr.as_ref()).and_then(|x| eval_unop(op, x)),
        &BinExpr(ref expr1, ref op, ref expr2) => {
            eval(time, expr1.as_ref()).and_then(|a| {
                eval(time, expr2.as_ref()).and_then(|b| eval_binop(op, a, b))
            })
        }
    }
}

fn eval_unop(op: &UnOp, value: i32) -> Result<i32, &'static str> {
    match op {
        &Neg => Ok(-value),
        &BoolNot => Ok(!value),
        &BitNot => Ok(!value),
    }
}

fn eval_binop(op: &BinOp, a: i32, b: i32) -> Result<i32, &'static str> {
    match op {
        &Two(BinOp2::Add) => Ok(a + b),
        &Two(BinOp2::Sub) => Ok(a - b),
        &One(BinOp1::Mul) => Ok(a * b),
        &One(BinOp1::Div) => {
            if b == 0 {
                Err("division by 0")
            } else {
                Ok(a / b)
            }
        },
        &Three(BitShift::Right) => Ok(a >> b),
        &Three(BitShift::Left) => Ok(a << b),
        &Four(BitAnd) => Ok(a & b),
        &Five(BitXOr) => Ok(a ^ b),
        &Six(BitOr) => Ok(a | b),
    }
}
