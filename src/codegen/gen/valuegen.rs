use koopa::ir::{Value, ValueKind};
use koopa::ir::values::{*};
use koopa::ir::entities::ValueData;
use crate::function_handler;
use crate::codegen::asmwriter::{AsmWriter};
use crate::codegen::context::ProgramContext;

use crate::codegen::Result;
use super::GenerateAsm;
use std::fs::File;


/*
impl GenerateAsm for Return {
    type Out = ();
    fn generate_asm(&self, f : &mut File, handler : &mut ProgramContext) -> crate::codegen::Result<Self::Out> {
        match self.value() {
            Some(v) => {

                writeln!(f,"  li a0, {}", 0)?;
            }
            None => ()
        };
        
        writeln!(f,"  ret")?;
        Ok(())
    }
}
*/

pub trait GenerateAsmValue<'p> {
    type Out;
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> Result<Self::Out>;
}
impl<'p> GenerateAsmValue<'p> for ValueData {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> Result<Self::Out> {
        match self.kind() {
            ValueKind::Jump(v) => v.generate_asm(f, context, value),
            ValueKind::Store(v) => v.generate_asm(f, context, value),
            ValueKind::Load(v) => v.generate_asm(f, context, value),
            ValueKind::Return(v) => v.generate_asm(f, context, value),
            ValueKind::Branch(v) => v.generate_asm(f, context, value),
            ValueKind::Binary(v) => v.generate_asm(f, context, value),
            /*
            
            ValueKind::Binary(v) => v.generate(f, info, self),
            ValueKind::Branch(v) => v.generate(f, info),
            ,
            */
            _ => Ok(())
        }
    }
}



impl<'p> GenerateAsmValue<'p> for Jump {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _v : &Value) -> Result<Self::Out> {
        let asm = AsmWriter::new();
        let name = function_handler!(context).get_basic_block_name(self.target());
        asm.jump(f, name);
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Store {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> Result<Self::Out> {
        let asm = AsmWriter::new();
        let tmp = "t2";
        asm.load(f, context, &(self.value()), tmp);
        asm.store(f, context, &(self.dest()), tmp);
        Ok(())
    }
}


impl<'p> GenerateAsmValue<'p> for Load {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> Result<Self::Out> {
        let asm = AsmWriter::new();
        let tmp = "t2";
        asm.load(f, context, &(self.src()), tmp);
        asm.store(f, context, &value, tmp);
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Return {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> Result<Self::Out> {
        let asm = AsmWriter::new();
        match self.value() {
            Some(v) => {
                asm.load(f,context, &v, "a0")?;
            },
            None => {}
        }
        
        asm.jump(f, &format!("end_{}", context.get_function_cnt()))?;
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Branch {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> Result<Self::Out> {
        let asm = AsmWriter::new();
        let tmp = "t2";
        asm.load(f, context, &(self.cond()), tmp)?;
        asm.beqz(f, context, tmp, self.false_bb())?;
        let true_name = function_handler!(context).get_basic_block_name(self.true_bb());
        asm.jump(f, true_name)?;
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Binary {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> Result<Self::Out> {
        let asmwriter = AsmWriter::new();
        asmwriter.load(f, context, &(self.lhs()), "t0");
        asmwriter.load(f, context, &(self.rhs()), "t1");
        let _tmp = "t2";
        match self.op() {
            BinaryOp::Add => asmwriter.add(f, "t0", "t1", "t2")?,
            BinaryOp::Sub => asmwriter.sub(f, "t0", "t1", "t2")?,
            BinaryOp::Div => asmwriter.div(f, "t0", "t1", "t2")?,
            BinaryOp::Mul => asmwriter.mul(f, "t0", "t1", "t2")?,
            BinaryOp::Mod => asmwriter.rem(f, "t0", "t1", "t2")?,
            BinaryOp::Lt => asmwriter.slt(f, "t0", "t1", "t2")?,
            BinaryOp::Gt => asmwriter.sgt(f, "t0", "t1","t2")?,
            BinaryOp::Eq => {
                asmwriter.xor(f, "t0", "t1", "t2");
                asmwriter.seqz(f, "t2", "t2");
            },
            BinaryOp::NotEq => {
                asmwriter.xor(f, "t0", "t1", "t2");
                asmwriter.snez(f, "t2", "t2");
            },
            BinaryOp::Le => {
                asmwriter.xor(f, "t0", "t1", "t2");
                asmwriter.seqz(f, "t2", "t2");
                asmwriter.slt(f, "t0", "t1", "t1");
                asmwriter.or(f, "t1", "t2", "t2");
            },
            BinaryOp::Ge => {
                asmwriter.xor(f, "t0", "t1", "t2");
                asmwriter.seqz(f, "t2", "t2");
                asmwriter.sgt(f, "t0", "t1", "t1");
                asmwriter.or(f, "t1", "t2", "t2");
            }
            _ => ()
        }
        asmwriter.store(f, context, value, "t2");
        Ok(())
    }
}