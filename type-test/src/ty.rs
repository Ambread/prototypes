use crate::{
    builder,
    data::Substitutions,
    error::{Error, Result},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Variable(String),
    Named(String),
    Func(Box<FuncTy>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncTy {
    pub from: Ty,
    pub to: Ty,
}

impl Ty {
    pub(crate) fn substitute(self, subs: &Substitutions) -> Ty {
        match self {
            Ty::Named(_) => self,
            Ty::Variable(ref name) => subs.get(name).cloned().unwrap_or(self),
            Ty::Func(func_ty) => {
                builder::ty_func(func_ty.from.substitute(subs), func_ty.to.substitute(subs))
            }
        }
    }

    pub(crate) fn unify(self, other: Ty) -> Result<Substitutions> {
        match (self, other) {
            (Ty::Named(x), Ty::Named(y)) if x == y => Ok(Substitutions::default()),
            (Ty::Variable(id), ty) => ty.bind_variable(id),
            (ty, Ty::Variable(id)) => ty.bind_variable(id),
            (Ty::Func(x), Ty::Func(y)) => {
                let mut subs = x.from.unify(y.from)?;

                subs += x.to.substitute(&subs).unify(y.to.substitute(&subs))?;

                Ok(subs)
            }
            (found, expected) => Err(Error::TypeMismatch { expected, found }),
        }
    }

    fn bind_variable(self, id: String) -> Result<Substitutions> {
        if matches!(self, Ty::Variable(ref var_id) if *var_id == id) {
            Ok(Substitutions::default())
        } else if self.contains_variable(&id) {
            Err(Error::SelfReference)
        } else {
            Ok(Substitutions::of(id, self))
        }
    }

    fn contains_variable(&self, id: &str) -> bool {
        match self {
            Ty::Named(_) => false,
            Ty::Variable(var_id) => var_id == id,
            Ty::Func(func_ty) => {
                func_ty.from.contains_variable(id) || func_ty.to.contains_variable(id)
            }
        }
    }

    pub fn try_into_func(self) -> Result<Box<FuncTy>, Self> {
        if let Self::Func(func) = self {
            Ok(func)
        } else {
            Err(self)
        }
    }
}
