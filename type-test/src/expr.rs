use crate::{ty::Ty, Context, Substitutions};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Variable(String),
    Func(FuncExpr),
    Call(CallExpr),
    If(IfExpr),
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
    pub body: Box<Expr>,
}

impl FuncExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let param_ty = ctx.new_ty_variable();
        let mut ctx = ctx.add(self.param, param_ty.clone());
        let (body_ty, subs) = self.body.infer(&mut ctx);
        let param_ty = param_ty.apply_subs(&subs);

        (
            Ty::Func {
                from: Box::new(param_ty),
                to: Box::new(body_ty),
            },
            subs,
        )
    }
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub func: Box<Expr>,
    pub arg: Box<Expr>,
}

impl CallExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let (func_ty, mut subs) = self.func.infer(ctx);
        let (arg_ty, new_subs) = self.arg.infer(&mut ctx.apply_subs(&subs));

        let new_var = ctx.new_ty_variable();
        subs += new_subs;

        let new_subs = Ty::Func {
            from: Box::new(arg_ty.clone()),
            to: Box::new(new_var),
        }
        .unify(func_ty.clone());
        let func_ty = func_ty.apply_subs(&new_subs);
        subs += new_subs;

        if let Ty::Func { from, to } = func_ty {
            subs += from.apply_subs(&subs).unify(arg_ty);

            (to.apply_subs(&subs), subs)
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct IfExpr {
    pub condition: Box<Expr>,
    pub true_branch: Box<Expr>,
    pub false_branch: Box<Expr>,
}

impl IfExpr {
    pub fn infer(self, ctx: &mut Context) -> (Ty, Substitutions) {
        let (condition_ty, subs0) = self.condition.infer(ctx);
        let mut subs = condition_ty.unify(Ty::Named("Bool".to_string()));

        let mut ctx = ctx.apply_subs(&(subs0 + subs.clone()));
        let (true_branch_ty, new_subs) = self.true_branch.infer(&mut ctx);
        subs += new_subs.clone();

        let mut ctx = ctx.apply_subs(&new_subs);
        let (false_branch_ty, new_subs) = self.false_branch.infer(&mut ctx);
        subs += new_subs;

        let true_branch_ty = true_branch_ty.apply_subs(&subs);
        let false_branch_ty = false_branch_ty.apply_subs(&subs);

        let new_subs = true_branch_ty.clone().unify(false_branch_ty);
        subs += new_subs.clone();

        (true_branch_ty.apply_subs(&new_subs), subs)
    }
}
