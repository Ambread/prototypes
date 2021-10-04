use crate::{
    ty::{FuncTy, Ty},
    Context, Substitutions,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Func(Box<FuncExpr>),
    Call(Box<CallExpr>),
    If(Box<IfExpr>),
}

impl Expr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        match self {
            Expr::Number(_) => (Ty::Named("Number".to_string()), Substitutions::default()),
            Expr::Variable(name) => (ctx.get(&name), Substitutions::default()),
            Expr::Func(it) => it.infer(ctx),
            Expr::Call(it) => it.infer(ctx),
            Expr::If(it) => it.infer(ctx),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncExpr {
    pub param: String,
    pub body: Expr,
}

impl FuncExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let param_ty = ctx.new_ty_variable();
        let mut ctx = ctx.with(self.param, param_ty.clone());
        let (body_ty, subs) = self.body.infer(&mut ctx);
        let param_ty = param_ty.substitute(&subs);

        (
            Ty::Func(Box::new(FuncTy {
                from: param_ty,
                to: body_ty,
            })),
            subs,
        )
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub func: Expr,
    pub arg: Expr,
}

impl CallExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let (func_ty, mut subs) = self.func.infer(ctx);
        let (arg_ty, new_subs) = self.arg.infer(&mut ctx.substitute(&subs));

        let new_var = ctx.new_ty_variable();
        subs += new_subs;

        let new_subs = Ty::Func(Box::new(FuncTy {
            from: arg_ty.clone(),
            to: new_var,
        }))
        .unify(func_ty.clone());
        let func_ty = func_ty.substitute(&new_subs);
        subs += new_subs;

        let func_ty = if let Ty::Func(func_ty) = func_ty {
            func_ty
        } else {
            unreachable!()
        };

        subs += func_ty.from.substitute(&subs).unify(arg_ty);

        (func_ty.to.substitute(&subs), subs)
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Expr,
    pub true_branch: Expr,
    pub false_branch: Expr,
}

impl IfExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let (condition_ty, mut condition_subs) = self.condition.infer(ctx);
        let mut subs = condition_ty.unify(Ty::Named("Bool".to_string()));
        condition_subs += subs.clone();

        let mut ctx = ctx.substitute(&condition_subs);
        let (true_branch_ty, new_subs) = self.true_branch.infer(&mut ctx);
        subs += new_subs.clone();

        let mut ctx = ctx.substitute(&new_subs);
        let (false_branch_ty, new_subs) = self.false_branch.infer(&mut ctx);
        subs += new_subs;

        let true_branch_ty = true_branch_ty.substitute(&subs);
        let false_branch_ty = false_branch_ty.substitute(&subs);

        let new_subs = true_branch_ty.clone().unify(false_branch_ty);
        subs += new_subs.clone();

        (true_branch_ty.substitute(&new_subs), subs)
    }
}
