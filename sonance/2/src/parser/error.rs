use std::num::ParseIntError;

use thiserror::Error;

pub type Result<T, E = ParseError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("string {0} is not a valid instruction")]
    InvalidInstruction(String),
    #[error("invalid constant")]
    InvalidConstant(#[from] ParseIntError),
    #[error("label {0} not found")]
    LabelNotFound(String),
}
