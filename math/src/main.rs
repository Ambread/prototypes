fn main() {
    let f = X.neg().neg();
    println!("{}", f.at(X));
}

use std::fmt::Display;

use Expr::{Const, X};

#[derive(Debug, Clone)]
enum Expr {
    X,
    Const(f64),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn at(self, x: Expr) -> Expr {
        use crate::{BinaryOp::*, Expr::*, UnaryOp::*};

        match self {
            X => x,
            Const(c) => Const(c),

            UnaryOp(op, a) => match (op, a.at(x)) {
                (Neg, Const(c)) => Const(-c),
                (Neg, UnaryOp(Neg, a)) => *a,
                (Sin, Const(c)) => Const(c.sin()),
                (Cos, Const(c)) => Const(c.cos()),
                (op, a) => UnaryOp(op, a.into()),
            },

            BinaryOp(op, a, b) => match (op, a.at(x.clone()), b.at(x)) {
                (Add, Const(a), Const(b)) => Const(a + b),
                (Sub, Const(a), Const(b)) => Const(a - b),
                (Mul, Const(a), Const(b)) => Const(a * b),
                (Div, Const(a), Const(b)) => Const(a / b),
                (Pow, Const(a), Const(b)) => Const(a.powf(b)),
                (op, a, b) => BinaryOp(op, a.into(), b.into()),
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
            Expr::BinaryOp(BinaryOp::Add, a, b) => write!(f, "({a} + {b})"),
            Expr::BinaryOp(BinaryOp::Sub, a, b) => write!(f, "({a} - {b})"),
            Expr::BinaryOp(BinaryOp::Mul, a, b) => write!(f, "({a} * {b})"),
            Expr::BinaryOp(BinaryOp::Div, a, b) => write!(f, "({a} / {b})"),
            Expr::BinaryOp(BinaryOp::Pow, a, b) => write!(f, "({a} ^ {b})"),
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

trait ToExpr: Sized {
    fn to_expr(self) -> Expr;

    fn neg(self) -> Expr {
        Expr::UnaryOp(UnaryOp::Neg, Box::new(self.to_expr()))
    }

    fn sin(self) -> Expr {
        Expr::UnaryOp(UnaryOp::Sin, Box::new(self.to_expr()))
    }

    fn cos(self) -> Expr {
        Expr::UnaryOp(UnaryOp::Cos, Box::new(self.to_expr()))
    }

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
