use std::ops::{Index, IndexMut};
use crate::models::Register;

#[derive(Debug, Default)]
pub struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Index<Register> for Registers {
    type Output = i32;

    fn index(&self, index: Register) -> &Self::Output {
        use Register::*;

        match index {
            A => &self.a,
            B => &self.b,
            C => &self.c,
            D => &self.d,
        }
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        use Register::*;

        match index {
            A => &mut self.a,
            B => &mut self.b,
            C => &mut self.c,
            D => &mut self.d,
        }
    }
}
