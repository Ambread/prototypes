#![feature(box_patterns)]

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
    BiOp(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Expr {
    fn at(self, x: impl ToExpr) -> Expr {
        match self {
            X => x.to_expr(),
            Expr::Const(c) => Expr::Const(c),
            Expr::UnaryOp(op, a) => Expr::UnaryOp(op, Box::new(a.at(x))),
            Expr::BiOp(a, op, b) => Expr::BiOp(Box::new(a.at(x.clone())), op, Box::new(b.at(x))),
        }
    }

    fn simplify(self) -> Expr {
        match self {
            X => X,
            Expr::Const(c) => Expr::Const(c),
            Expr::UnaryOp(UnaryOp::Neg, box Expr::UnaryOp(UnaryOp::Neg, box a)) => a,
            Expr::UnaryOp(op, a) => Expr::UnaryOp(op, a),
            Expr::BiOp(a, op, b) => Expr::BiOp(a, op, b),
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
            Expr::BiOp(a, BinaryOp::Add, b) => write!(f, "{a} + {b}"),
            Expr::BiOp(a, BinaryOp::Sub, b) => write!(f, "{a} - {b}"),
            Expr::BiOp(a, BinaryOp::Mul, b) => write!(f, "{a} * {b}"),
            Expr::BiOp(a, BinaryOp::Div, b) => write!(f, "{a} / {b}"),
            Expr::BiOp(a, BinaryOp::Pow, b) => write!(f, "{a} ^ {b}"),
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
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Add,
            Box::new(rhs.to_expr()),
        )
    }

    fn sub(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Sub,
            Box::new(rhs.to_expr()),
        )
    }

    fn mul(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Mul,
            Box::new(rhs.to_expr()),
        )
    }

    fn div(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Div,
            Box::new(rhs.to_expr()),
        )
    }

    fn pow(self, rhs: impl ToExpr) -> Expr {
        Expr::BiOp(
            Box::new(self.to_expr()),
            BinaryOp::Pow,
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
