#![allow(dead_code)]

use std::{
    collections::HashMap,
    ops::{Add, AddAssign},
};

fn main() {
    let mut env = HashMap::new();
    env.insert("even?".to_owned(), tfunc(tn("Number"), tn("Bool")));
    env.insert("inc".to_owned(), tfunc(tn("Number"), tn("Number")));

    dbg!(infer(
        c(v("even?"), c(v("inc"), i(123))),
        &mut Context::new(env)
    ));
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

fn v(name: &str) -> Expression {
    Expression::Variable(name.to_owned())
}

fn i(number: i32) -> Expression {
    Expression::Number(number)
}

fn c(func: Expression, arg: Expression) -> Expression {
    Expression::Call {
        func: Box::new(func),
        arg: Box::new(arg),
    }
}

#[derive(Debug, Clone)]
enum Expression {
    Number(i32),
    Variable(String),
    Func {
        param: String,
        body: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        arg: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        true_branch: Box<Expression>,
        false_branch: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
enum Ty {
    Named(String),
    Variable(String),
    Func { from: Box<Ty>, to: Box<Ty> },
}

impl Ty {
    fn contains(&self, name: &str) -> bool {
        match self {
            Ty::Named(_) => false,
            Ty::Variable(var_name) => var_name == name,
            Ty::Func { from, to } => from.contains(name) || to.contains(name),
        }
    }
}

fn infer(expr: Expression, ctx: &mut Context) -> (Ty, Substitutions) {
    match expr {
        Expression::Number(_) => (Ty::Named("Number".to_string()), Substitutions::default()),
        Expression::Variable(name) => (ctx.get(&name), Substitutions::default()),
        Expression::Func { param, body } => {
            let param_ty = ctx.new_ty_variable();
            let mut ctx = ctx.add(param, param_ty.clone());
            let (body_ty, subs) = infer(*body, &mut ctx);
            let param_ty = apply_subs_to_ty(&subs, param_ty);

            (
                Ty::Func {
                    from: Box::new(param_ty),
                    to: Box::new(body_ty),
                },
                subs,
            )
        }
        Expression::Call { func, arg } => {
            let (func_ty, mut subs) = infer(*func, ctx);
            let (arg_ty, new_subs) = infer(*arg, &mut apply_subs_to_context(&subs, ctx));

            let new_var = ctx.new_ty_variable();
            subs += new_subs;

            let new_subs = unify(
                Ty::Func {
                    from: Box::new(arg_ty.clone()),
                    to: Box::new(new_var),
                },
                func_ty.clone(),
            );
            let func_ty = apply_subs_to_ty(&new_subs, func_ty);
            subs += new_subs;

            if let Ty::Func { from, to } = func_ty {
                subs += unify(apply_subs_to_ty(&subs, *from), arg_ty);

                (apply_subs_to_ty(&subs, *to), subs)
            } else {
                unreachable!()
            }
        }
        Expression::If {
            condition,
            true_branch,
            false_branch,
        } => {
            let (condition_ty, subs0) = infer(*condition, ctx);
            let mut subs = unify(condition_ty, Ty::Named("Bool".to_string()));

            let mut ctx1 = apply_subs_to_context(&(subs0 + subs.clone()), ctx);
            let (true_branch_ty, new_subs) = infer(*true_branch, &mut ctx1);
            subs += new_subs.clone();

            let mut ctx2 = apply_subs_to_context(&new_subs, &ctx1);
            let (false_branch_ty, new_subs) = infer(*false_branch, &mut ctx2);
            subs += new_subs;

            let true_branch_ty = apply_subs_to_ty(&subs, true_branch_ty);
            let false_branch_ty = apply_subs_to_ty(&subs, false_branch_ty);

            let new_subs = unify(true_branch_ty.clone(), false_branch_ty);
            subs += new_subs.clone();

            (apply_subs_to_ty(&new_subs, true_branch_ty), subs)
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Context {
    env: HashMap<String, Ty>,
    next: usize,
}

impl Context {
    fn new(env: HashMap<String, Ty>) -> Self {
        Self { env, next: 0 }
    }

    fn get(&self, name: &str) -> Ty {
        self.env.get(name).expect("Unbound type variable").clone()
    }

    fn new_ty_variable(&mut self) -> Ty {
        let ty = Ty::Variable(format!("T{}", self.next));
        self.next += 1;
        ty
    }

    fn add(&self, name: String, ty: Ty) -> Context {
        let mut context = self.clone();
        context.env.insert(name, ty);
        context
    }
}

#[derive(Debug, Clone, Default)]
struct Substitutions(HashMap<String, Ty>);

impl Add for Substitutions {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.0.extend(rhs.0.into_iter());
        self
    }
}

impl AddAssign for Substitutions {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0.into_iter())
    }
}

fn apply_subs_to_ty(subs: &Substitutions, ty: Ty) -> Ty {
    match ty {
        Ty::Named(_) => ty,
        Ty::Variable(ref name) => subs.0.get(name).cloned().unwrap_or(ty),
        Ty::Func { from, to } => Ty::Func {
            from: Box::new(apply_subs_to_ty(subs, *from)),
            to: Box::new(apply_subs_to_ty(subs, *to)),
        },
    }
}

fn unify(x: Ty, y: Ty) -> Substitutions {
    match (x, y) {
        (Ty::Named(x), Ty::Named(y)) if x == y => Substitutions::default(),
        (Ty::Variable(x), y) => var_bind(x, y),
        (x, Ty::Variable(y)) => var_bind(y, x),
        (
            Ty::Func {
                from: x_from,
                to: x_to,
            },
            Ty::Func {
                from: y_from,
                to: y_to,
            },
        ) => {
            let mut subs = unify(*x_from, *y_from);

            subs += unify(
                apply_subs_to_ty(&subs, *x_to),
                apply_subs_to_ty(&subs, *y_to),
            );

            subs
        }
        _ => panic!("Type mismatch"),
    }
}

fn var_bind(name: String, ty: Ty) -> Substitutions {
    if matches!(ty, Ty::Variable(ref ty_name) if *ty_name == name) {
        Substitutions::default()
    } else if ty.contains(&name) {
        panic!("Type contains self reference")
    } else {
        let mut subs = Substitutions::default();
        subs.0.insert(name, ty);
        subs
    }
}

fn apply_subs_to_context(subs: &Substitutions, ctx: &Context) -> Context {
    Context::new(
        ctx.env
            .clone()
            .into_iter()
            .map(|(name, ty)| (name, apply_subs_to_ty(subs, ty)))
            .collect(),
    )
}
