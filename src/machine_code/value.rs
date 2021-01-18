use std::convert::TryInto;

use crate::models::Value;

use super::{Compile, Decompile, DecompileResult, EndOfInput, OutputError};

impl Decompile for Value {
    type Error = EndOfInput;

    fn decompile(bytes: &[u8]) -> Result<DecompileResult<Self>, Self::Error> {
        if bytes.len() < 4 {
            Err(EndOfInput { name: "Value" })?
        }

        if let Ok(bytes) = bytes[..4].try_into() {
            let value = i32::from_be_bytes(bytes);
            let result = DecompileResult {
                value: Value(value),
                bytes_read: 4,
            };
            Ok(result)
        } else {
            unreachable!()
        }
    }
}

impl Compile for Value {
    type Error = OutputError;

    fn compile(&self, output: &mut impl std::io::Write) -> Result<(), Self::Error> {
        output.write(self.0.to_be_bytes().as_ref())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_bytes() {
        let value = Value::from(42);
        let mut bytes = Vec::new();
        value.compile(&mut bytes).unwrap();
        let value_again = Value::decompile(&bytes[..]).unwrap().value;
        assert_eq!(value, value_again)
    }

    #[test]
    fn from_three_bytes() {
        let bytes: &[u8] = &[0; 3];
        assert!(Value::decompile(bytes).is_err());
    }

    #[test]
    fn from_five_bytes() {
        let bytes: &[u8] = &[0; 5];
        assert_eq!(4, Value::decompile(bytes).unwrap().bytes_read);
    }
}
