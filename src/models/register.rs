use std::{fmt::Display, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;

        match s.trim().to_ascii_uppercase().as_str() {
            "A" => Ok(A),
            "B" => Ok(B),
            "C" => Ok(C),
            "D" => Ok(D),
            x => Err(format!("Unknown register: {}", x)),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;

        let result = match self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
        };
        write!(f, "{}", result)
    }
}
