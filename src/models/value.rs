use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Value(pub i32);

impl From<i32> for Value {
    fn from(x: i32) -> Self {
        Value(x)
    }
}

impl Value {
    pub fn value(self) -> i32 {
        self.0
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse().map_err(|_| format!("Incorrect integer: {}", s))?;
        Ok(Value(number))
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
