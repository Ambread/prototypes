fn main() {
    let f = 6.mul(X.exp(2)).add(17.mul(X)).add(12);
    dbg!(f.eval(5.0));
}

use Expr::X;

enum Expr {
    X,
    Const(f64),
    UnaryOp(UnaryOp, Box<Expr>),
    BiOp(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Expr {
    fn eval(self, x: f64) -> f64 {
        match self {
            X => x,
            Expr::Const(c) => c,
            Expr::UnaryOp(op, a) => op.eval(a.eval(x)),
            Expr::BiOp(a, op, b) => op.eval(a.eval(x), b.eval(x)),
        }
    }
}

enum UnaryOp {
    Neg,
    Sin,
    Cos,
}

impl UnaryOp {
    fn eval(self, a: f64) -> f64 {
        match self {
            UnaryOp::Neg => -a,
            UnaryOp::Sin => a.sin(),
            UnaryOp::Cos => a.cos(),
        }
    }
}

enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl BinaryOp {
    fn eval(self, a: f64, b: f64) -> f64 {
        match self {
            BinaryOp::Add => a + b,
            BinaryOp::Sub => a - b,
            BinaryOp::Mul => a * b,
            BinaryOp::Div => a / b,
            BinaryOp::Pow => a.powf(b),
        }
    }
}

trait ToExpr: Sized {
    fn to_expr(self) -> Expr;

    fn add(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }

    fn sub(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }

    fn mul(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }

    fn div(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }

    fn exp(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }
}

impl ToExpr for Expr {
    fn to_expr(self) -> Expr {
        self
    }
}

impl ToExpr for f64 {
    fn to_expr(self) -> Expr {
        Expr::Const(self)
    }
}

impl ToExpr for i32 {
    fn to_expr(self) -> Expr {
        Expr::Const(self as f64)
    }
}
