mod valuegen;


use valuegen::GenerateAsmValue;
use koopa::ir::{FunctionData, Program, ValueKind, Type};
use super::asmwriter::AsmWriter;
use super::{CResult};
use super::context::ProgramContext;
use super::func::FunctionHandler;
use std::fs::File;
use std::cmp;

// A unified trait for all the memory structure in Program

// Program are composed by value and function
pub trait GenerateAsm<'p> {
    type Out;
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> CResult<Self::Out>;
}

impl<'p> GenerateAsm<'p> for Program {
    type Out = ();

    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> CResult<Self::Out> {
        

        // generate function
        for &func in self.func_layout() {
            context.func = Some(func);
            // generate assembly for FunctionData
            self.func(func).generate_asm(f, context)?;
        }
        Ok(())
    }
}

impl<'p> GenerateAsm<'p> for FunctionData{
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext) -> CResult<Self::Out> {
        // check if it is the library function
        if self.layout().entry_bb().is_none() {
            return Ok(());
        }

        // build asmwriter for the file pointer
        let mut asmwriter = AsmWriter::new();
        
        // 1. generate prologue for the function
        
        //let cur_func = context.get_func_handler().unwarp();

        let (all_size, r_size) = {
            let mut s_size : i32 = 0;
            let mut r_size : i32 = 0;
            let mut a_size : i32 = 0;
            for (_bb, node) in self.layout().bbs() {
                for &value in node.insts().keys() {
                    let d = context.get_value_data(&value);
                    match d.kind() {
                        ValueKind::Binary(_) | ValueKind::Alloc(_) | ValueKind::Load(_)=> {s_size += 4;}
                        ValueKind::Call(t) => {
                            a_size = cmp::max(a_size, (t.args().len() as i32) - 8);
                            
                            r_size = 4;
                            s_size += 4;
                        }
                        _ => {}
                    };
                }
            }
            a_size = a_size * 4;
            let mut total = s_size + a_size + r_size;
            if total == 0 {(0, r_size)}
            else {
                (
                    ((total - 1) / 16 + 1) * 16,
                    r_size
                )
            }
        };
        asmwriter.prologue(f, &(self.name()), all_size, r_size)?;
        
        context.set_current_function(
            FunctionHandler::new(context.get_function_cnt(), self.params().to_owned(), all_size as usize)
        );

        // mark every element position in the function handler
        let mut alloc_pos = all_size - r_size - 4;
        for (_bb, node) in self.layout().bbs() {
            for &value in node.insts().keys() {
                let d = context.get_value_data(&value);
                match d.kind() {
                    ValueKind::Binary(_) | ValueKind::Alloc(_) | ValueKind::Load(_) |
                    ValueKind::Call(_) => {
                        context.set_function_value_pos(value, alloc_pos as usize);
                        alloc_pos -= 4;
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
        asmwriter.epilogue(f, context.get_function_cnt(), all_size, r_size)?;
        Ok(())
    }
}


