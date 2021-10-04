use crate::Substitutions;

#[derive(Debug, Clone)]
pub enum Ty {
    Named(String),
    Variable(String),
    Func(Box<FuncTy>),
}

#[derive(Debug, Clone)]
pub struct FuncTy {
    pub from: Ty,
    pub to: Ty,
}

impl Ty {
    pub(crate) fn substitute(self, subs: &Substitutions) -> Ty {
        match self {
            Ty::Named(_) => self,
            Ty::Variable(ref name) => subs.0.get(name).cloned().unwrap_or(self),
            Ty::Func(func_ty) => Ty::Func(Box::new(FuncTy {
                from: func_ty.from.substitute(subs),
                to: func_ty.to.substitute(subs),
            })),
        }
    }

    pub(crate) fn unify(self, y: Ty) -> Substitutions {
        match (self, y) {
            (Ty::Named(x), Ty::Named(y)) if x == y => Substitutions::default(),
            (Ty::Variable(x), y) => y.var_bind(x),
            (x, Ty::Variable(y)) => x.var_bind(y),
            (Ty::Func(x), Ty::Func(y)) => {
                let mut subs = x.from.unify(y.from);

                subs += x.to.substitute(&subs).unify(y.to.substitute(&subs));

                subs
            }
            _ => panic!("Type mismatch"),
        }
    }

    fn var_bind(self, name: String) -> Substitutions {
        if matches!(self, Ty::Variable(ref ty_name) if *ty_name == name) {
            Substitutions::default()
        } else if self.contains(&name) {
            panic!("Type contains self reference")
        } else {
            let mut subs = Substitutions::default();
            subs.0.insert(name, self);
            subs
        }
    }

    fn contains(&self, name: &str) -> bool {
        match self {
            Ty::Named(_) => false,
            Ty::Variable(var_name) => var_name == name,
            Ty::Func(func_ty) => func_ty.from.contains(name) || func_ty.to.contains(name),
        }
    }
}
