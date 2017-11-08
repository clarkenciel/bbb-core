use eval::eval;
use expr::Expr;
use sample::signal::Signal;

pub struct ExprSignal {
    time: i32,
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
    type Frame = [i32; 1];

    fn next(&mut self) -> Self::Frame {
        if let Ok(x) = eval(self.time, &self.expression) {
            self.time +=1;
            [x]
        } else {
            self.time += 1;
            [0]
        }
    }
}
