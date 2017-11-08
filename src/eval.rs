use expr::Expr;
use numeral::Numeral::*;
use ops::*;
use self::Expr::*;
use self::UnOp::*;
use self::BinOp::*;

pub fn eval(time: u32, expression: &Expr) -> Result<i32, &'static str> {
    match expression {
        &Time => Ok(time as i32),
        &Num(Int(i)) => Ok(i as i32),
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
        &Neg => Ok(0),
        &BoolNot => Ok(!(value as i32) as i32),
        &BitNot => Ok(!(value as i32) as i32),
    }
}

fn eval_binop(op: &BinOp, a: i32, b: i32) -> Result<i32, &'static str> {
    match op {
        &Two(BinOp2::Add) => Ok(a + b),
        &Two(BinOp2::Sub) => Ok(a - b),
        &One(BinOp1::Mul) => Ok(a * b),
        &One(BinOp1::Div) => {
            if b as i32 == 0 {
                Err("division by 0")
            } else {
                Ok(a / b)
            }
        },
        &Three(BitShift::Right) => Ok(((a as i32) >> (b as i32)) as i32),
        &Three(BitShift::Left) => Ok(((a as i32) << (b as i32)) as i32),
        &Four(BitAnd) => Ok(((a as i32) & (b as i32)) as i32),
        &Five(BitXOr) => Ok(((a as i32) ^ (b as i32)) as i32),
        &Six(BitOr) => Ok(((a as i32) | (b as i32)) as i32),
    }
}
