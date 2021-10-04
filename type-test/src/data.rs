use std::{collections::HashMap, ops::AddAssign};

use crate::{
    error::{Error, Result},
    ty::Ty,
};

#[derive(Debug, Clone, Default)]
pub struct Context {
    env: HashMap<String, Ty>,
    next: usize,
}

impl Context {
    pub fn new(env: HashMap<String, Ty>) -> Self {
        Self { env, next: 0 }
    }

    pub fn get(&self, name: &str) -> Result<&Ty> {
        self.env
            .get(name)
            .ok_or_else(|| Error::UnboundTypeVariable {
                name: name.to_owned(),
            })
    }

    pub fn new_ty_variable(&mut self) -> Ty {
        let ty = Ty::Variable(format!("T{}", self.next));
        self.next += 1;
        ty
    }

    pub fn with(&self, name: String, ty: Ty) -> Self {
        let mut context = self.clone();
        context.env.insert(name, ty);
        context
    }

    pub fn substitute(&self, subs: &Substitutions) -> Self {
        Self::new(
            self.env
                .clone()
                .into_iter()
                .map(|(name, ty)| (name, ty.substitute(subs)))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct Substitutions(HashMap<String, Ty>);

impl Substitutions {
    pub fn get(&self, name: &str) -> Option<&Ty> {
        self.0.get(name)
    }

    pub fn of(name: String, ty: Ty) -> Substitutions {
        let mut map = HashMap::with_capacity(1);
        map.insert(name, ty);
        Substitutions(map)
    }
}

impl AddAssign for Substitutions {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0.into_iter())
    }
}
