use std::{convert::TryFrom, fmt::Display, str::FromStr};

use super::*;

pub struct Assembly(Vec<Statement>);

impl TryFrom<Vec<Statement>> for Assembly {
    type Error = String;
    fn try_from(statements: Vec<Statement>) -> Result<Self, Self::Error> {
        Ok(Assembly(statements))
    }
}

impl Assembly {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.0
    }
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

impl FromStr for Assembly {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .enumerate()
            .filter(|&(_, s)| !s.is_empty())
            .map(|(i, s)| s.parse().map_err(|msg| (i, msg)));


            
        let mut assembly = Vec::new();
        let mut errors = Vec::new();
        for line in lines {
            match line {
                Ok(line) => {
                    if errors.is_empty() {
                        assembly.push(line)
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if errors.is_empty() {
            Ok(Assembly(assembly))
        } else {
            let error = errors
                .iter()
                .map(|(i, err)| format!("Line {}: {}", i + 1, err))
                .collect::<Vec<String>>()
                .join(LINE_ENDING);
            Err(error)
        }
    }
}

impl Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
