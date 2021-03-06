use eval::eval;
use expr::Expr;
use sample::signal::Signal;

#[derive(Clone)]
pub struct ExprSignal {
    pub time: i32,
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
    type Frame = [i8; 1];

    fn next(&mut self) -> Self::Frame {
        if let Ok(x) = eval(self.time, &self.expression) {
            self.time +=1;
            [x as i8]
        } else {
            self.time += 1;
            [0]
        }
    }
}
