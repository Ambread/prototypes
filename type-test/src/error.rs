use thiserror::Error;

use crate::ty::Ty;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Type mismatch")]
    TypeMismatch { expected: Ty, found: Ty },
    #[error("Type contains self reference")]
    SelfReference { ty: Ty },
    #[error("Unbound type variable")]
    UnboundTypeVariable { name: String },
}
