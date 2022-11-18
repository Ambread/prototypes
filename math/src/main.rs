fn main() {
    let f = X.divided(1);
    println!("{}", f.at(X));
}

use std::fmt::Display;

use crate::{BinaryOp::*, Error::*, Expr::*, UnaryOp::*};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    X,
    Const(f64),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
    Error(Error),
}

impl Expr {
    fn at(self, x: Expr) -> Expr {
        match self {
            X => x,
            Const(c) => Const(c),
            Error(e) => Error(e),

            UnaryOp(op, a) => match (op, a.at(x)) {
                (Neg, Const(c)) => Const(-c),
                (Neg, UnaryOp(Neg, a)) => *a,
                (op, a) => UnaryOp(op, a.into()),
            },

            BinaryOp(op, a, b) => match (op, a.at(x.clone()), b.at(x)) {
                (Add, Const(a), Const(b)) => Const(a + b),
                (Add, a, b) if a == Const(0.0) => b,
                (Add, a, b) if b == Const(0.0) => a,

                (Mul, Const(a), Const(b)) => Const(a * b),
                (Mul, a, b) if a == Const(1.0) => b,
                (Mul, a, b) if b == Const(1.0) => a,
                (Mul, a, b) if a == Const(0.0) || b == Const(0.0) => Const(0.0),

                (Div, _, b) if b == Const(0.0) => Error(DivByZero),
                (Div, Const(a), Const(b)) => Const(a / b),
                (Div, a, b) if b == Const(1.0) => a,

                (Pow, Const(a), Const(b)) => Const(a.powf(b)),
                (Pow, a, b) if b == Const(1.0) => a,
                (Pow, _, b) if b == Const(0.0) => Const(0.0),

                (op, a, b) => BinaryOp(op, a.into(), b.into()),
            },
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X => write!(f, "x"),
            Const(c) => write!(f, "{c}"),
            UnaryOp(Neg, a) => write!(f, "-{a}"),
            UnaryOp(Sin, a) => write!(f, "sin({a})"),
            UnaryOp(Cos, a) => write!(f, "cos({a})"),
            BinaryOp(Add, a, b) => write!(f, "({a} + {b})"),
            BinaryOp(Mul, a, b) => write!(f, "({a} * {b})"),
            BinaryOp(Div, a, b) => write!(f, "({a} / {b})"),
            BinaryOp(Pow, a, b) => write!(f, "({a} ^ {b})"),
            Error(DivByZero) => write!(f, "<DivByZero>"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum UnaryOp {
    Neg,
    Sin,
    Cos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BinaryOp {
    Add,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DivByZero,
}

pub fn neg(expr: impl ToExpr) -> Expr {
    Expr::UnaryOp(UnaryOp::Neg, Box::new(expr.to_expr()))
}

pub fn sin(expr: impl ToExpr) -> Expr {
    Expr::UnaryOp(UnaryOp::Sin, Box::new(expr.to_expr()))
}

pub fn cos(expr: impl ToExpr) -> Expr {
    Expr::UnaryOp(UnaryOp::Cos, Box::new(expr.to_expr()))
}

pub trait ToExpr: Sized {
    fn to_expr(self) -> Expr;

    fn plus(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Add,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn times(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Mul,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn divided(self, rhs: impl ToExpr) -> Expr {
        Expr::BinaryOp(
            BinaryOp::Div,
            Box::new(self.to_expr()),
            Box::new(rhs.to_expr()),
        )
    }

    fn to(self, rhs: impl ToExpr) -> Expr {
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
