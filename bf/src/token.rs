use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter, Result as FmtResult},
};

pub fn scan(input: impl Iterator<Item = char>) -> impl Iterator<Item = Token> {
    input.filter_map(|char| char.try_into().ok())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    Open,
    Close,
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(char: char) -> Result<Self, Self::Error> {
        Ok(match char {
            '>' => Self::Right,
            '<' => Self::Left,
            '+' => Self::Inc,
            '-' => Self::Dec,
            '.' => Self::Output,
            ',' => Self::Input,
            '[' => Self::Open,
            ']' => Self::Close,
            _ => return Err(()),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Right => write!(f, ">"),
            Self::Left => write!(f, "<"),
            Self::Inc => write!(f, "+"),
            Self::Dec => write!(f, "-"),
            Self::Output => write!(f, "."),
            Self::Input => write!(f, ","),
            Self::Open => write!(f, "["),
            Self::Close => write!(f, "]"),
        }
    }
}
