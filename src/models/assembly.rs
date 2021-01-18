use std::{fmt::Display, str::FromStr};

use super::*;

pub struct Assembly(Vec<Statement>);

impl Assembly {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.0
    }

    pub fn new(statements: Vec<Statement>) -> Assembly {
        Assembly(statements)
    }
}

impl FromStr for Assembly {
    type Err = AssemblyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let statements = s
            .lines()
            .enumerate()
            .filter(|&(_, s)| !s.is_empty())
            .map(|(line, s)| s.parse().map_err(|error| LineWithError { line, error }));

        let (assembly, errors) = partition_results(statements);

        if errors.is_empty() {
            Ok(Assembly(assembly))
        } else {
            Err(AssemblyParseError { errors })
        }
    }
}

fn partition_results<T, U>(results: impl Iterator<Item = Result<T, U>>) -> (Vec<T>, Vec<U>) {
    let mut oks = Vec::new();
    let mut errors = Vec::new();
    for result in results {
        match result {
            Ok(x) => {
                if errors.is_empty() {
                    oks.push(x)
                }
            }
            Err(x) => errors.push(x),
        }
    }
    (oks, errors)
}

impl Display for Assembly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
