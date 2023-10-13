use std::fs::File;
use std::io::Write;
use crate::codegen::Result;
use crate::function_handler;
use koopa::ir::{Value,ValueKind, BasicBlock};

use super::context::ProgramContext;
pub struct AsmWriter{}


macro_rules! binary_op {
    ($name:ident) => {
        pub fn $name(&self, f: &mut File, rs1: &str, rs2: &str, rd: &str) -> Result<()>{
            let op_name = stringify!($name);
            writeln!(f,"{}",format!("{} {}, {}, {}",op_name, rd, rs1, rs2))?;
            Ok(())
        }
    };
}
impl AsmWriter {
    pub fn new() -> Self {
        Self { }
    }

    pub fn prologue(&mut self, f : &mut File, func_name : &str, stack_size : i32)  -> Result<()>{
        writeln!(f,"  .text")?;
        writeln!(f,"  .globl {}", &func_name[1..])?;
        writeln!(f,"{}:", &func_name[1..])?;
        if stack_size != 0 {
            writeln!(f, "addi sp, sp, {}", -stack_size);
        }
        Ok(())
    }
    pub fn epilogue(&mut self, f: &mut File, stack_size : i32, func_id : usize) -> Result<()> {
        writeln!(f, "end_{}:", func_id);
        if stack_size != 0 {
            writeln!(f, "addi sp, sp, {}", stack_size)?;
        }
        writeln!(f, "ret")?;
        Ok(())

    }
    pub fn block_name(&self, f: &mut File, name : &str) -> Result<()> {
        writeln!(f, "{name}:")?;
        Ok(())
    }

    pub fn jump(&self, f: &mut File, block_name: &str) -> Result<()> {
        writeln!(f,"j {}", block_name)?;
        Ok(())
    }

    pub fn load(&self, f: &mut File, context: &mut ProgramContext, value : &Value, tmp: &str) -> Result<()> {
        match context.get_value_data(value).kind() {
            ValueKind::Integer(v) => {
                writeln!(f,"li {}, {}", tmp, v.value())?;
            }
            _ => {
                let pos = function_handler!(context).get_value_pos(value)?;
                writeln!(f,"lw {}, {}(sp)", tmp, pos)?;
            }
        }
        Ok(())
    }
    pub fn store(&self, f: &mut File, context: &mut ProgramContext, value : &Value, tmp: &str) -> Result<()> {
        let pos = function_handler!(context).get_value_pos(value)?;
        writeln!(f, "sw {}, {}(sp)", tmp, pos)?;
        Ok(())
    }
    //REMEMBER: Always use BasicBlock directly, not reference
    pub fn beqz(&self, f: &mut File, context: &mut ProgramContext, rs: &str, false_bb: BasicBlock) -> Result<()>{
        let fbb_name = function_handler!(context).get_basic_block_name(false_bb);
        writeln!(f, "beqz {}, {}", rs, fbb_name)?;
        Ok(())
    }

    // write integer val
    pub fn li(&self, f: &mut File, rd: &str, val: i32) -> Result<()>{
        writeln!(f, "li {}, {}", rd, val)?;
        Ok(())
    }

    // compare register with zero
    pub fn seqz(&self, f: &mut File, rs: &str, rd: &str) -> Result<()>{
        writeln!(f, "seqz {}, {}", rd, rs)?;
        Ok(())
    }

    // compare register with zero
    pub fn snez(&self, f: &mut File, rs: &str, rd: &str) -> Result<()>{
        writeln!(f, "snez {}, {}", rd, rs)?;
        Ok(())
    }
    // BinaryOp
    binary_op!(add);
    binary_op!(sub);
    binary_op!(mul);
    binary_op!(div);
    binary_op!(rem);
    binary_op!(slt);
    binary_op!(sgt);
    binary_op!(or);
    binary_op!(xor);
    binary_op!(and);

}