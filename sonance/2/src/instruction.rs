use casey::snake;
use std::{fmt::Display, str::FromStr};

use crate::parser::ParseError;

macro_rules! generate_instructions {
    (enum $ty:ident { $($ident:ident,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $ty {
            $($ident,)*
        }

        impl From<$ty> for u8 {
            fn from(value: $ty) -> Self {
                value as u8
            }
        }

        impl TryFrom<u8> for $ty {
            type Error = ();

            #[allow(non_upper_case_globals)]
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                $(const $ident: u8 = $ty::$ident as u8;)*
                Ok(match value {
                    $($ident => $ty::$ident,)*
                    _ => return Err(()),
                })
            }
        }

        impl Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($ty::$ident => write!(f, snake!(stringify!($ident))),)*
                }
            }
        }

        impl FromStr for $ty {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(snake!(stringify!($ident)) => $ty::$ident,)*
                    _ => return Err(ParseError::InvalidInstruction(s.into())),
                })
            }
        }
    };
}

generate_instructions! {
    enum Instruction {
        Halt,
        Debug,
        Noop,

        Push,
        PushU16,
        PushU32,
        PushU64,
        Pop,
        Dupe,

        Jump,
        JumpIf,

        Load,
        Store,

        Call,
        Return,

        Add,
        Sub,
        Mul,
        Div,

        BitAnd,
        BitOr,
        BitNot,

        BoolAnd,
        BoolOr,
        BoolNot,

        Eq,
        Gt,
        Geq,
    }
}
