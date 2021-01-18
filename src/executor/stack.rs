use super::ExecutionError;

#[derive(Default, Debug)]
pub struct Stack {
    data: Vec<i32>,
}

impl Stack {
    pub fn push(&mut self, value: i32) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, ExecutionError> {
        self.data.pop().ok_or(ExecutionError::StackUnderflow)
    }
}
