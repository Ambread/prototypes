use std::{fmt::Display, str::FromStr};

use crate::parser::ParseError;

macro_rules! instruction {
    (enum $ty:ident { $($number:literal $ident:ident $name:ident,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $ty {
            $($ident,)*
        }

        impl From<$ty> for u8 {
            fn from(value: $ty) -> Self {
                match value {
                    $($ty::$ident => $number,)*
                }
            }
        }

        impl TryFrom<u8> for $ty {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Ok(match value {
                    $($number => $ty::$ident,)*
                    _ => return Err(()),
                })
            }
        }

        impl Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($ty::$ident => write!(f, stringify!($name)),)*
                }
            }
        }

        impl FromStr for $ty {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $(stringify!($name) => $ty::$ident,)*
                    _ => return Err(ParseError::InvalidInstruction(s.into())),
                })
            }
        }
    };
}

instruction! {
    enum Instruction {
        0 Halt halt,

        1 Push push,
        2 Pop pop,
        3 Dupe dupe,

        4 Jump jump,
        5 JumpIf jump_if,

        6 Load load,
        7 Store store,

        8 Call call,
        9 Return return,

        10 Add add,
        11 Sub sub,
        12 Mul mul,
        13 Div div,

        14 BitAnd bit_and,
        15 BitOr bit_or,
        16 BitNot bit_not,

        17 BoolAnd bool_and,
        18 BoolOr bool_or,
        19 BoolNot bool_not,

        20 Eq eq,
        21 Gt gt,
        22 Geq geq,
    }
}
