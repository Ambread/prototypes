fn main() {
    let f = 6.mul(X.pow(2)).add(17.mul(X)).add(12);
    println!("{}", f.at(5.0));
}

use std::fmt::Display;

use Expr::X;

#[derive(Debug, Clone)]
enum Expr {
    X,
    Const(f64),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn at(self, x: impl ToExpr) -> Expr {
        match self {
            X => x.to_expr(),
            Expr::Const(c) => Expr::Const(c),
            Expr::UnaryOp(op, a) => Expr::UnaryOp(op, Box::new(a.at(x))),
            Expr::BinaryOp(op, a, b) => {
                Expr::BinaryOp(op, Box::new(a.at(x.clone())), Box::new(b.at(x)))
            }
        }
        .simplify()
    }

    fn simplify(self) -> Expr {
        use crate::{BinaryOp::*, Expr::*, UnaryOp::*};

        match self {
            X => X,
            Const(c) => Const(c),

            UnaryOp(op, a) => match a.simplify() {
                Const(c) => Const(match op {
                    Neg => -c,
                    Sin => c.sin(),
                    Cos => c.cos(),
                }),
                a => UnaryOp(op, a.into()),
            },

            BinaryOp(op, a, b) => match (a.simplify(), b.simplify()) {
                (Const(a), Const(b)) => Const(match op {
                    Add => a + b,
                    Sub => a - b,
                    Mul => a * b,
                    Div => a / b,
                    Pow => a.powf(b),
                }),
                (a, b) => BinaryOp(op, a.into(), b.into()),
            },
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X => write!(f, "x"),
            Expr::Const(c) => write!(f, "{c}"),
            Expr::UnaryOp(UnaryOp::Neg, a) => write!(f, "-{a}"),
            Expr::UnaryOp(UnaryOp::Sin, a) => write!(f, "sin({a})"),
            Expr::UnaryOp(UnaryOp::Cos, a) => write!(f, "cos({a})"),
            Expr::BinaryOp(BinaryOp::Add, a, b) => write!(f, "{a} + {b}"),
            Expr::BinaryOp(BinaryOp::Sub, a, b) => write!(f, "{a} - {b}"),
            Expr::BinaryOp(BinaryOp::Mul, a, b) => write!(f, "{a} * {b}"),
            Expr::BinaryOp(BinaryOp::Div, a, b) => write!(f, "{a} / {b}"),
            Expr::BinaryOp(BinaryOp::Pow, a, b) => write!(f, "{a} ^ {b}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum UnaryOp {
    Neg,
    Sin,
    Cos,
}

#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

trait ToExpr: Sized + Clone {
    fn to_expr(self) -> Expr;

    fn add(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Add,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn sub(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Sub,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn mul(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Mul,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn div(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Div,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn pow(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Pow,
            Box::new(self.to_expr()),
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
