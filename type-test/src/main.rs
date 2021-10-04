#![allow(dead_code)]

use std::collections::HashMap;

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

fn infer(expr: Expression, ctx: &mut Context) -> (Ty, Substitutions) {
    match expr {
        Expression::Number(_) => (Ty::Named("Number".to_string()), HashMap::new()),
        Expression::Variable(name) => (ctx.get(&name), HashMap::new()),
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
            let (func_ty, subs1) = infer(*func, ctx);
            let (arg_ty, subs2) = infer(*arg, &mut apply_subs_to_context(&subs1, ctx));

            let new_var = ctx.new_ty_variable();
            let subs3 = compose_subs(subs1, subs2);
            let subs4 = unify(
                Ty::Func {
                    from: Box::new(arg_ty.clone()),
                    to: Box::new(new_var),
                },
                func_ty.clone(),
            );

            let func_ty = apply_subs_to_ty(&subs4, func_ty);
            let subs5 = compose_subs(subs3, subs4);

            if let Ty::Func { from, to } = func_ty {
                let subs6 = unify(apply_subs_to_ty(&subs5, *from), arg_ty);
                let subs7 = compose_subs(subs5, subs6);
                (apply_subs_to_ty(&subs7, *to), subs7)
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
            let subs1 = unify(condition_ty, Ty::Named("Bool".to_string()));

            let mut ctx1 = apply_subs_to_context(&compose_subs(subs0, subs1.clone()), ctx);
            let (true_branch_ty, subs2) = infer(*true_branch, &mut ctx1);
            let subs3 = compose_subs(subs1, subs2.clone());

            let mut ctx2 = apply_subs_to_context(&subs2, &ctx1);
            let (false_branch_ty, subs4) = infer(*false_branch, &mut ctx2);
            let subs5 = compose_subs(subs3, subs4);

            let true_branch_ty = apply_subs_to_ty(&subs5, true_branch_ty);
            let false_branch_ty = apply_subs_to_ty(&subs5, false_branch_ty);

            let subs6 = unify(true_branch_ty.clone(), false_branch_ty);
            let subs7 = compose_subs(subs5, subs6.clone());

            (apply_subs_to_ty(&subs6, true_branch_ty), subs7)
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

type Substitutions = HashMap<String, Ty>;

fn apply_subs_to_ty(subs: &Substitutions, ty: Ty) -> Ty {
    match ty {
        Ty::Named(_) => ty,
        Ty::Variable(ref name) => subs.get(name).cloned().unwrap_or(ty),
        Ty::Func { from, to } => Ty::Func {
            from: Box::new(apply_subs_to_ty(subs, *from)),
            to: Box::new(apply_subs_to_ty(subs, *to)),
        },
    }
}

fn unify(x: Ty, y: Ty) -> Substitutions {
    match (x, y) {
        (Ty::Named(x), Ty::Named(y)) if x == y => HashMap::new(),
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
            let subs1 = unify(*x_from, *y_from);
            let subs2 = unify(
                apply_subs_to_ty(&subs1, *x_to),
                apply_subs_to_ty(&subs1, *y_to),
            );

            compose_subs(subs1, subs2)
        }
        _ => panic!("Type mismatch"),
    }
}

fn compose_subs(mut x: Substitutions, y: Substitutions) -> Substitutions {
    x.extend(y.into_iter());
    x
}

fn var_bind(name: String, ty: Ty) -> Substitutions {
    if matches!(ty, Ty::Variable(ref ty_name) if *ty_name == name) {
        HashMap::new()
    } else if contains(&ty, &name) {
        panic!("Type contains self reference")
    } else {
        let mut subs = HashMap::new();
        subs.insert(name, ty);
        subs
    }
}

fn contains(ty: &Ty, name: &str) -> bool {
    match ty {
        Ty::Named(_) => false,
        Ty::Variable(var_name) => var_name == name,
        Ty::Func { from, to } => contains(from, name) || contains(to, name),
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
