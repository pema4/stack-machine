use crate::models::{Assembly, Op, Statement};

use super::*;

impl Decompile for Assembly {
    type Error = AssemblyDecompileError;

    fn decompile(bytes: &[u8]) -> Result<DecompileResult<Self>, Self::Error> {
        let mut idx = 0;
        let mut result = Vec::new();

        while idx < bytes.len() {
            let op = Op::decompile(&bytes[idx..])?; // todo: unwrap
            result.push(Statement::Op(op.value));
            idx += op.bytes_read;
        }

        Ok(DecompileResult {
            value: Assembly::new(result),
            bytes_read: idx,
        })
    }
}

impl Compile for Assembly {
    type Error = OutputError;

    fn compile(&self, output: &mut impl std::io::Write) -> Result<(), Self::Error> {
        for statement in self.statements() {
            match statement {
                Statement::Op(op) => op.compile(output)?,
            }
        }
        Ok(())
    }
}
