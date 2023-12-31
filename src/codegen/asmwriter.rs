use std::fs::File;
use std::io::Write;
use crate::codegen::CResult;
use crate::function_handler;
use koopa::ir::{Value,ValueKind, BasicBlock, Function};
use crate::codegen::func::FuncVar;
use super::{context::ProgramContext, GenerateError};
pub struct AsmWriter{}

const SPACE : &str = "  ";
macro_rules! binary_op {
    ($name:ident) => {
        pub fn $name(&self, f: &mut File, rs1: &str, rs2: &str, rd: &str) -> CResult<()>{
            let op_name = stringify!($name);
            writeln!(f,"{SPACE}{}",format!("{} {}, {}, {}",op_name, rd, rs1, rs2))?;
            Ok(())
        }
    };
}
#[allow(unused)]
impl AsmWriter {
    pub fn new() -> Self {
        Self { }
    }

    pub fn prologue(&mut self, f : &mut File, func_name : &str, stack_size : i32, r_size : i32)  -> CResult<()>{
        writeln!(f,"")?;
        writeln!(f,"{SPACE}.text")?;
        writeln!(f,"{SPACE}.globl {}", &func_name[1..])?;
        writeln!(f,"{}:", &func_name[1..])?;
        if stack_size != 0 {
            writeln!(f, "{SPACE}addi sp, sp, {}", -stack_size)?;
        }
        if r_size != 0 {
            writeln!(f, "{SPACE}sw ra, {}(sp)", stack_size - 4);
        }
        Ok(())
    }
    pub fn epilogue(&mut self, f: &mut File, func_id : usize, stack_size : i32, r_size : i32) -> CResult<()> {
        writeln!(f, "end_{}:", func_id)?;
        if r_size != 0 {
            writeln!(f, "{SPACE}lw ra, {}(sp)", stack_size - 4)?;
        }
        if stack_size != 0 {
            writeln!(f, "{SPACE}addi sp, sp, {}", stack_size)?;
        }
        
        writeln!(f, "{SPACE}ret")?;
        Ok(())

    }

    pub fn block_name(&self, f: &mut File, name : &str) -> CResult<()> {
        writeln!(f, "{name}:")?;
        Ok(())
    }

    pub fn call(&self, f: &mut File, context: &mut ProgramContext, func : Function) -> CResult<()> {
        let name = context.get_function_name(func);
        writeln!(f, "{SPACE}call {}",&name[1..])?;
        Ok(())
    }
    pub fn jump(&self, f: &mut File, block_name: &str) -> CResult<()> {
        writeln!(f,"{SPACE}j {}", block_name)?;
        Ok(())
    }

    pub fn load(&self, f: &mut File, context: &mut ProgramContext, value : &Value, tmp: &str) -> CResult<()> {
        if let Some(v) = context.search_global_var(value) {
            writeln!(f, "{SPACE}la {}, {}", tmp, v);
            writeln!(f, "{SPACE}lw {}, 0({})", tmp, tmp);
            return Ok(());
        }
        
        match context.get_value_data(value).kind() {
            ValueKind::Integer(v) => {
                writeln!(f,"{SPACE}li {}, {}", tmp, v.value())?;
            }
            _ => {
                let res  = context.get_value_position(value)?;
                match res {
                    FuncVar::Reg(s) => {
                        writeln!(f, "{SPACE}mv {}, {}", tmp, s)?;
                    },
                    FuncVar::Stack(p) => {
                        writeln!(f, "{SPACE}lw {}, {}(sp)", tmp, p)?;
                    }
                }
            }
        }
        Ok(())
    }
    pub fn store(&self, f: &mut File, context: &mut ProgramContext, value : &Value, tmp: &str) -> CResult<()> {
        if let Some(v) = context.search_global_var(value) {
            writeln!(f, "la t3, {}",  v);
            writeln!(f, "sw {}, 0(t3)", tmp);
            return Ok(());
        }
        let pos = context.get_value_position(value)?;
        match pos {
            FuncVar::Stack(p) => {
                self.store_to_stack(f, tmp, p)
            },
            _ => {
                Err(GenerateError::NoValue)
            }
        }
    }

    pub fn store_to_stack(&self, f:&mut File, tmp : &str, pos : usize) -> CResult<()> {
        writeln!(f, "{SPACE}sw {}, {}(sp)", tmp, pos)?;
        Ok(())
    }

    //REMEMBER: Always use BasicBlock directly, not reference
    pub fn beqz(&self, f: &mut File, context: &mut ProgramContext, rs: &str, false_bb: BasicBlock) -> CResult<()>{
        let fbb_name = function_handler!(context).get_basic_block_name(false_bb);
        writeln!(f, "{SPACE}beqz {}, {}", rs, fbb_name)?;
        Ok(())
    }

    // write integer val
    pub fn li(&self, f: &mut File, rd: &str, val: i32) -> CResult<()>{
        writeln!(f, "{SPACE}li {}, {}", rd, val)?;
        Ok(())
    }

    // compare register with zero
    pub fn seqz(&self, f: &mut File, rs: &str, rd: &str) -> CResult<()>{
        writeln!(f, "{SPACE}seqz {}, {}", rd, rs)?;
        Ok(())
    }

    // compare register with zero
    pub fn snez(&self, f: &mut File, rs: &str, rd: &str) -> CResult<()>{
        writeln!(f, "{SPACE}snez {}, {}", rd, rs)?;
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