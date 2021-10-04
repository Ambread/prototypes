use std::collections::HashMap;

use type_test::{
    expr::{self, Expr},
    ty::Ty,
    Context,
};

fn main() {
    let mut env = HashMap::new();
    env.insert("even?".to_owned(), tfunc(tn("Number"), tn("Bool")));
    env.insert("inc".to_owned(), tfunc(tn("Number"), tn("Number")));

    let e = c(v("even?"), c(v("inc"), i(123)));

    dbg!(e.infer(&mut Context::new(env)));
}

fn tfunc(from: Ty, to: Ty) -> Ty {
    Ty::Func {
        from: Box::new(from),
        to: Box::new(to),
    }
}

fn tn(name: &str) -> Ty {
    Ty::Named(name.to_owned())
}

fn v(name: &str) -> Expr {
    Expr::Variable(name.to_owned())
}

fn i(number: i32) -> Expr {
    Expr::Number(number)
}

fn c(func: Expr, arg: Expr) -> Expr {
    Expr::Call(expr::CallExpr {
        func: Box::new(func),
        arg: Box::new(arg),
    })
}
