use eval::eval;
use expr::Expr;
use sample::signal::Signal;

pub struct ExprSignal {
    time: u32,
    expression: Expr,
}

impl From<Expr> for ExprSignal {
    fn from(expr: Expr) -> ExprSignal {
        ExprSignal {
            time: 0,
            expression: expr,
        }
    }
}

impl Signal for ExprSignal {
    type Frame = [f32; 1];

    fn next(&mut self) -> Self::Frame {
        match eval(self.time, &self.expression) {
            Err(_) => [0.0],
            Ok(x) => [x],
        }
    }
}
