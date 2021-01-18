use std::{self, error, fmt};
use thiserror::Error;

/// Statement parse error and line number, where this error occured.
///
/// Part of the `AssemblyParseError` struct.
#[derive(Debug, PartialEq, Eq)]
pub struct LineWithError {
    /// Line where error occured.
    ///
    /// Starts from 0.
    pub line: usize,

    /// Inner error
    pub error: StatementParseError,
}

/// Collection of assembly parse errors.
#[derive(Debug, PartialEq, Eq)]
pub struct AssemblyParseError {
    /// Each error is a tuple
    pub errors: Vec<LineWithError>,
}

impl fmt::Display for AssemblyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} errors occured when parsing assembly:",
            self.errors.len()
        )?;
        for error in self.errors.iter() {
            writeln!(f, "Line {}: {}", error.line + 1, error.error)?;
        }
        Ok(())
    }
}

impl error::Error for AssemblyParseError {}

/// An error that may occur when parsing assembly's statement.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum StatementParseError {
    #[error("{0}")]
    OpError(#[from] OpParseError),
}

/// An error that may occur when parsing operation and it's arguments.
#[derive(Debug, PartialEq, Eq)]
pub enum OpParseError {
    /// Wrong arguments for the operation.
    WrongArguments {
        op: &'static str,
        errors: Vec<(usize, ArgumentParseError)>,
    },
    /// Unknown op or wrong number of arguments.
    WrongOp { op: String, num_args: usize },
}

impl fmt::Display for OpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpParseError::WrongArguments { op, errors } => {
                writeln!(f, "Errors occured when parsing operation {}:", op)?;
                for (idx, error) in errors {
                    let suffix = match idx + 1 {
                        1 => "st",
                        2 => "nd",
                        3 => "rd",
                        _ => "th",
                    };
                    writeln!(f, "  {}{} argument: {}", idx + 1, suffix, error)?;
                }
                Ok(())
            }
            OpParseError::WrongOp { op, num_args } => {
                write!(f, "Unknown operation {} with {} arguments", op, num_args)
            }
        }
    }
}

impl error::Error for OpParseError {}

/// An error that may occur when parsing arguments to the operation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ArgumentParseError {
    /// Expected register, but found something else.
    #[error("Expected register A, B, C or D, got: {0}")]
    WrongRegister(String),

    /// Expected value, but found something else.
    #[error("Expected integer with radix 10, got: {0}")]
    WrongValue(String),

    /// Expected register or value, but found something else.
    #[error("Expected integer or register, got: {0}")]
    WrongRegisterOrValue(String),
}
