use thiserror::Error;

use crate::vm::Instruction;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("instruction {0:?} wanted a value from the stack, but it was empty")]
    EmptyStack(Instruction),
    #[error("attempted to return outside of function call")]
    TopLevelReturn,
    #[error("attempted to access frame, but none exist")]
    ExpectedFrame,
}
