mod valuegen;
use koopa::ir::entities::ValueData;
use koopa::ir::values::*;
use koopa::ir::{BasicBlock, Function, FunctionData, Program, TypeKind, Value, ValueKind};
use super::Result;
use super::prog::ProgramHandler;
use std::fs::File;
use std::io::Write;

// A unified trait for all the memory structure in Program

// Program are composed by value and function
pub trait GenerateAsm {
    type Out;
    fn generate_asm(&self, f : &mut File, handler : &mut ProgramHandler) -> Result<Self::Out>;
}

impl GenerateAsm for Program {
    type Out = ();

    fn generate_asm(&self, f : &mut File, handler : &mut ProgramHandler) -> Result<Self::Out> {
        

        // generate function
        for &func in self.func_layout() {
            // save current function in the handler to pass infomation
            handler.set_current_function(func);
            // generate assembly for FunctionData
            self.func(func).generate_asm(f, handler)?;
        }
        Ok(())
    }
}

impl GenerateAsm for FunctionData{
    type Out = ();
    fn generate_asm(&self, f : &mut File, handler : &mut ProgramHandler) -> Result<Self::Out> {
        // 1. generate prologue for the function

        writeln!(f, "  .text")?;
        let name = self.name();
        writeln!(f, "  .globl {}", &name[1..])?;
        writeln!(f, "{}:", &name[1..])?;

        // 2. generate instruction in basic blocks

        for (bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys() {
                self.dfg().value(inst).generate_asm(f, handler);
            }
        }
        Ok(())
    }
}

impl GenerateAsm for ValueData {
    type Out = ();
    fn generate_asm(&self, f : &mut File, handler : &mut ProgramHandler) -> Result<Self::Out> {
        match self.kind() {
            ValueKind::Return(v) => v.generate_asm(f, handler),
            _ => Ok(())
        }
    }
}
