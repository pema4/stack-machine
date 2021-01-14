use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ExitCode {
    Success,
    StackUnderflow,
    InputError,
    OutputError,
    IncorrectJump,
}

impl Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExitCode::Success => write!(f, "Program terminated successfully"),
            ExitCode::StackUnderflow => write!(f, "Stack underflow"),
            ExitCode::InputError => write!(f, "Can't read from input stream"),
            ExitCode::OutputError => write!(f, "Can't write to output stream"),
            ExitCode::IncorrectJump => write!(f, "Incorrect jump"),
        }
    }
}

impl From<ExitCode> for String {
    fn from(code: ExitCode) -> Self {
        code.to_string()
    }
}