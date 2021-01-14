use super::ExitCode;

#[derive(Default, Debug)]
pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, ExitCode> {
        self.data.pop().ok_or(ExitCode::StackUnderflow)
    }
}
