use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Expr {
    Value(Value),
    Var(String),

    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),

    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    Fn {
        name: String,
        args: Vec<String>,
        body: Box<Expr>,
        then: Box<Expr>,
    },
}

pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let ident = text::ident().padded();

    let expr = recursive(|expr| {
        let int = text::int(10)
            .map(|s: String| Value::Num(s.parse().unwrap()))
            .padded();

        let bool = text::keyword("true")
            .to(Value::Bool(true))
            .or(text::keyword("false").to(Value::Bool(false)));

        let value = int.map(Expr::Value).or(bool.map(Expr::Value));

        let call = ident
            .then(
                expr.clone()
                    .separated_by(just(','))
                    .allow_trailing()
                    .delimited_by(just('('), just(')')),
            )
            .map(|(f, args)| Expr::Call(f, args));

        let atom = value
            .or(expr.delimited_by(just('('), just(')')))
            .or(call)
            .or(ident.map(Expr::Var));

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op('/').to(Expr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op('-').to(Expr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum.padded()
    });

    let decl = recursive(|decl| {
        let r#let = text::keyword("let")
            .ignore_then(ident)
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl.clone())
            .map(|((name, rhs), then)| Expr::Let {
                name,
                rhs: Box::new(rhs),
                then: Box::new(then),
            });

        let r#fn = text::keyword("fn")
            .ignore_then(ident)
            .then(ident.repeated())
            .then_ignore(just('='))
            .then(expr.clone())
            .then_ignore(just(';'))
            .then(decl)
            .map(|(((name, args), body), then)| Expr::Fn {
                name,
                args,
                body: Box::new(body),
                then: Box::new(then),
            });

        r#let.or(r#fn).or(expr).padded()
    });

    decl.then_ignore(end())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Num(f64),
    Bool(bool),
}

impl Value {
    fn as_num(&self) -> Result<f64, String> {
        if let Self::Num(v) = self {
            Ok(*v)
        } else {
            Err("Wrong type".to_owned())
        }
    }
}

impl Expr {
    pub fn eval<'a>(
        &'a self,
        vars: &mut Vec<(&'a String, Value)>,
        funcs: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
    ) -> Result<Value, String> {
        eval(self, vars, funcs)
    }
}

pub fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(&'a String, Value)>,
    funcs: &mut Vec<(&'a String, &'a [String], &'a Expr)>,
) -> Result<Value, String> {
    match expr {
        Expr::Value(x) => Ok(*x),

        Expr::Neg(a) => Ok(Value::Num(-a.eval(vars, funcs)?.as_num()?)),
        Expr::Add(a, b) => Ok(Value::Num(
            a.eval(vars, funcs)?.as_num()? + b.eval(vars, funcs)?.as_num()?,
        )),
        Expr::Sub(a, b) => Ok(Value::Num(
            a.eval(vars, funcs)?.as_num()? - b.eval(vars, funcs)?.as_num()?,
        )),
        Expr::Mul(a, b) => Ok(Value::Num(
            a.eval(vars, funcs)?.as_num()? * b.eval(vars, funcs)?.as_num()?,
        )),
        Expr::Div(a, b) => Ok(Value::Num(
            a.eval(vars, funcs)?.as_num()? / b.eval(vars, funcs)?.as_num()?,
        )),

        Expr::Var(name) => {
            if let Some((_, val)) = vars.iter().rev().find(|(var, _)| *var == name) {
                Ok(*val)
            } else {
                Err(format!("Cannot find variable `{}` in scope", name))
            }
        }

        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((name, rhs));
            let output = eval(then, vars, funcs);
            vars.pop();
            output
        }

        Expr::Call(name, args) => {
            let (_, arg_names, body) =
                if let Some(t) = funcs.iter().rev().find(|(var, _, _)| *var == name).copied() {
                    t
                } else {
                    return Err(format!("Cannot find function `{}` in scope", name));
                };

            if arg_names.len() != args.len() {
                return Err(format!(
                    "Wrong number of arguments for function `{}`: expected {}, found {}",
                    name,
                    arg_names.len(),
                    args.len(),
                ));
            }

            let mut args = args
                .iter()
                .map(|arg| eval(arg, vars, funcs))
                .zip(arg_names.iter())
                .map(|(val, name)| Ok((name, val?)))
                .collect::<Result<_, String>>()?;

            vars.append(&mut args);
            let output = eval(body, vars, funcs);
            vars.truncate(vars.len() - args.len());
            output
        }

        Expr::Fn {
            name,
            args,
            body,
            then,
        } => {
            funcs.push((name, args, body));
            let output = eval(then, vars, funcs);
            funcs.pop();
            output
        }
    }
}
