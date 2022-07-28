use thiserror::Error;

use crate::vm::Instruction;

pub type Result<T, E = VMError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Error)]
pub enum VMError {
    #[error("instruction {0} at index {1} wanted a value from the stack, but it was empty")]
    EmptyStack(Instruction, u8),
    #[error("attempted to return at index {0} outside of function call")]
    TopLevelReturn(u8),
    #[error("instruction {0} at index {1} attempted to access frame, but none exist")]
    ExpectedFrame(Instruction, u8),
    #[error("attempted to access instruction index {0}, but it was out of bounds")]
    InstructionIndexOutOfBounds(u8),
    #[error("{0} at index {1} is not a valid instruction")]
    InvalidInstruction(u8, u8),
}
