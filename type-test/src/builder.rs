use crate::{
    data::Context,
    error::{Error, Result},
    expr::{CallExpr, Expr, FuncExpr, IfExpr},
    ty::{FuncTy, Ty},
};

pub fn infer(expr: Expr) -> Result<Ty> {
    let env = [
        (
            "even?".to_owned(),
            ty_func(ty_name("Number"), ty_name("Bool")),
        ),
        (
            "inc".to_owned(),
            ty_func(ty_name("Number"), ty_name("Number")),
        ),
    ];

    let mut context = Context::new(IntoIterator::into_iter(env).collect());

    expr.infer(&mut context).map(|it| it.0)
}

pub fn ty_name(name: &str) -> Ty {
    Ty::Named(name.to_owned())
}

pub fn ty_func(from: Ty, to: Ty) -> Ty {
    Ty::Func(Box::new(FuncTy { from, to }))
}

pub fn number(number: i32) -> Expr {
    Expr::Number(number)
}

pub fn var(name: &str) -> Expr {
    Expr::Variable(name.to_owned())
}

pub fn func(param: &str, body: Expr) -> Expr {
    Expr::Func(Box::new(FuncExpr {
        param: param.to_owned(),
        body,
    }))
}

pub fn call(func: Expr, arg: Expr) -> Expr {
    Expr::Call(Box::new(CallExpr { func, arg }))
}

pub fn if_else(condition: Expr, true_branch: Expr, false_branch: Expr) -> Expr {
    Expr::If(Box::new(IfExpr {
        condition,
        true_branch,
        false_branch,
    }))
}

pub fn err_mismatch(expected: Ty, found: Ty) -> Error {
    Error::TypeMismatch { expected, found }
}
