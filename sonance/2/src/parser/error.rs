use std::num::ParseIntError;

use thiserror::Error;

pub type Result<T, E = ParseError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid constant")]
    InvalidConstant(#[from] ParseIntError),
}
