use crate::token::Token;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub fn parse(input: impl Iterator<Item = Token>) -> Vec<Expr> {
    let mut output = vec![vec![]];

    for token in input {
        let token = match token {
            Token::Right => Expr::Right,
            Token::Left => Expr::Left,
            Token::Inc => Expr::Inc,
            Token::Dec => Expr::Dec,
            Token::Output => Expr::Output,
            Token::Input => Expr::Input,
            Token::Open => {
                output.push(vec![]);
                continue;
            }
            Token::Close => Expr::Loop(output.pop().expect("Stack to not be empty")),
        };

        output
            .last_mut()
            .expect("Stack to not be empty")
            .push(token);
    }

    if output.len() == 1 {
        output.remove(0)
    } else {
        panic!("Stack to have exactly one left")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Right,
    Left,
    Inc,
    Dec,
    Output,
    Input,
    Loop(Vec<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Right => write!(f, ">"),
            Self::Left => write!(f, "<"),
            Self::Inc => write!(f, "+"),
            Self::Dec => write!(f, "-"),
            Self::Output => write!(f, "."),
            Self::Input => write!(f, ","),
            Self::Loop(body) => {
                write!(f, "[")?;
                for item in body {
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
        }
    }
}
