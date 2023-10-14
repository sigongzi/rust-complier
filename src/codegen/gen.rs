mod valuegen;


use valuegen::GenerateAsmValue;
use koopa::ir::{FunctionData, Program, ValueKind};
use super::asmwriter::AsmWriter;
use super::{Result};
use super::context::ProgramContext;
use super::func::FunctionHandler;
use std::fs::File;


// A unified trait for all the memory structure in Program

// Program are composed by value and function
pub trait GenerateAsm<'p> {
    type Out;
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> Result<Self::Out>;
}

impl<'p> GenerateAsm<'p> for Program {
    type Out = ();

    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> Result<Self::Out> {
        

        // generate function
        for &func in self.func_layout() {
            // save current function in the handler to pass infomation
            context.set_current_function(
                FunctionHandler::new(func, context.get_function_cnt())
            );


            // generate assembly for FunctionData
            self.func(func).generate_asm(f, context)?;
        }
        Ok(())
    }
}

impl<'p> GenerateAsm<'p> for FunctionData{
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> Result<Self::Out> {
        
        // build asmwriter for the file pointer
        let mut asmwriter = AsmWriter::new();
        
        // 1. generate prologue for the function
        
        //let cur_func = context.get_func_handler().unwarp();
        let stack_size = {
            let mut sum : i32 = 0;
            for (_bb, node) in self.layout().bbs() {
                for &value in node.insts().keys() {
                    let d = context.get_value_data(&value);
                    match d.kind() {
                        ValueKind::Binary(_) | ValueKind::Alloc(_) | ValueKind::Load(_)=> {sum += 4;}
                        _ => {}
                    };
                }
            }
            if sum == 0 {0}
            else {
                ((sum -  1)/ 16 + 1) * 16
            }
        };
        asmwriter.prologue(f, &(self.name()), stack_size)?;

        let mut alloc_pos = 0;
        for (_bb, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let d = context.get_value_data(&value);
                match d.kind() {
                    ValueKind::Binary(_) | ValueKind::Alloc(_) | ValueKind::Load(_)=> {
                        context.cur_func.as_mut().unwrap().set_value_pos(value, alloc_pos);
                        alloc_pos += 4;
                    }
                    _ => {}
                };
            }
        }
        

        // 2. generate instruction in basic blocks
        
        for (bb, node) in self.layout().bbs() {
            asmwriter.block_name(f, context.get_basic_block_name(*bb))?;
            for &inst in node.insts().keys() {
                self.dfg().value(inst).generate_asm(f, context, &inst)?;
            }
        }
        asmwriter.epilogue(f, stack_size, context.get_function_cnt())?;
        Ok(())
    }
}


