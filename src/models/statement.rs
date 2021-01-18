use std::{fmt::Display, str::FromStr};

use super::{Op, StatementParseError};

pub enum Statement {
    Op(Op),
}

impl FromStr for Statement {
    type Err = StatementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = s.parse()?;
        Ok(Statement::Op(op))
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Statement::*;

        match self {
            Op(op) => write!(f, "{}", op),
        }
    }
}
