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

#[cfg(test)]
mod test {

    use super::*;
    use crate::models::*;

    fn test_compile_decompile(statements: Vec<Statement>) {
        let asm = Assembly::new(statements);
        let mut machine_code = Vec::new();
        asm.compile(&mut machine_code).unwrap();
        let asm_again = Assembly::decompile(machine_code.as_ref()).unwrap().value;
        let pairs_iter = asm.statements().iter().zip(asm_again.statements());
        for (left, right) in pairs_iter {
            let Statement::Op(left) = left;
            let Statement::Op(right) = right;
            assert_eq!(left, right);
        }
    }

    #[test]
    fn one_plus_three() {
        test_compile_decompile(vec![
            Statement::Op(Op::PushValue(Value(1))),
            Statement::Op(Op::PushValue(Value(2))),
            Statement::Op(Op::Add),
        ]);
    }

    #[test]
    fn calculate_answer() {
        test_compile_decompile(vec![
            Statement::Op(Op::PushValue(Value(41))),
            Statement::Op(Op::PushValue(Value(1))),
            Statement::Op(Op::Add),
        ]);
    }
}
