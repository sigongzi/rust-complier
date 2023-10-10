use koopa::ir::{Value, ValueKind};
use koopa::ir::values::{Return, Integer};

use crate::codegen::prog::ProgramHandler;
use crate::codegen::{Result, GenerateError};
use super::GenerateAsm;
use std::fs::File;
use std::io::Write;

impl GenerateAsm for Return {
    type Out = ();
    fn generate_asm(&self, f : &mut File, handler : &mut ProgramHandler) -> crate::codegen::Result<Self::Out> {
        let func_id = handler.cur_func().ok_or(GenerateError::MissingCurrentFunction("return".to_string()))?;

        let value_data = handler.program().func(func_id).dfg()
        .value(
            self.value().ok_or(GenerateError::Unimplemented)?
        );
        let v = match value_data.kind() {
            ValueKind::Integer(v) => v.value(),
            _ => {
                return Err(GenerateError::Unimplemented);
            }
        };
        writeln!(f,"  li a0, {}", v)?;
        writeln!(f,"  ret")?;
        Ok(())
    }
}