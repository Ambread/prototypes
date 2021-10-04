pub mod builder;
pub mod expr;
pub mod ty;

use std::{collections::HashMap, ops::AddAssign};

use ty::Ty;

#[derive(Debug, Clone, Default)]
pub struct Context {
    env: HashMap<String, Ty>,
    next: usize,
}

impl Context {
    pub fn new(env: HashMap<String, Ty>) -> Self {
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

    fn with(&self, name: String, ty: Ty) -> Self {
        let mut context = self.clone();
        context.env.insert(name, ty);
        context
    }

    fn substitute(&self, subs: &Substitutions) -> Self {
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

impl AddAssign for Substitutions {
    fn add_assign(&mut self, rhs: Self) {
        self.0.extend(rhs.0.into_iter())
    }
}
