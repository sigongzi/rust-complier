
use koopa::ir::{Value, ValueKind};
use koopa::ir::values::{*};
use koopa::ir::entities::ValueData;
use crate::function_handler;
use crate::codegen::asmwriter::{AsmWriter};
use crate::codegen::context::ProgramContext;
use crate::codegen::CResult;
#[allow(unused_imports)]
use super::GenerateAsm;
use std::fs::File;
use std::io::Write;


pub trait GenerateAsmValue<'p> {
    type Out;
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> CResult<Self::Out>;
}
impl<'p> GenerateAsmValue<'p> for ValueData {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> CResult<Self::Out> {
        match self.kind() {
            ValueKind::Jump(v) => v.generate_asm(f, context, value),
            ValueKind::Store(v) => v.generate_asm(f, context, value),
            ValueKind::Load(v) => v.generate_asm(f, context, value),
            ValueKind::Return(v) => v.generate_asm(f, context, value),
            ValueKind::Branch(v) => v.generate_asm(f, context, value),
            ValueKind::Binary(v) => v.generate_asm(f, context, value),
            ValueKind::Call(v) => v.generate_asm(f, context, value),
            ValueKind::GlobalAlloc(v) => v.generate_asm(f, context, value),
            /*
            
            ValueKind::Binary(v) => v.generate(f, info, self),
            ValueKind::Branch(v) => v.generate(f, info),
            ,
            */
            _ => Ok(())
        }
    }
}

impl<'p> GenerateAsmValue<'p> for GlobalAlloc {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> CResult<Self::Out> {
        match context.program.borrow_value(self.init()).kind() {
            ValueKind::Integer(a) => {
                writeln!(f, "  .word {}", a.value())?;
            },
            ValueKind::ZeroInit(_z) => {
                writeln!(f, "  .zero 4")?;
            },
            _ => unreachable!()
        };

        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Jump {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _v : &Value) -> CResult<Self::Out> {
        let asm = AsmWriter::new();
        let name = function_handler!(context).get_basic_block_name(self.target());
        asm.jump(f, name)?;
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Store {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> CResult<Self::Out> {
        let asm = AsmWriter::new();
        let tmp = "t2";
        asm.load(f, context, &(self.value()), tmp)?;
        asm.store(f, context, &(self.dest()), tmp)?;
        Ok(())
    }
}


impl<'p> GenerateAsmValue<'p> for Load {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> CResult<Self::Out> {
        let asm = AsmWriter::new();
        let tmp = "t2";
        asm.load(f, context, &(self.src()), tmp)?;
        asm.store(f, context, &value, tmp)?;
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Return {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> CResult<Self::Out> {
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

impl<'p> GenerateAsmValue<'p> for Call {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> CResult<Self::Out> {
        let asm = AsmWriter::new();
        for (id, param) in self.args().iter().take(8).enumerate() {
            asm.load(f, context, param, format!("a{}",id).as_str())?;
        }

        for (id, param) in self.args().iter().skip(8).enumerate() {
            asm.load(f, context, param, "t0")?;
            asm.store_to_stack(f, "t0", id * 4)?;
        }
        asm.call(f, context, self.callee())?;
        // println!("the name of function is {}", context.get_function_name(self.callee()) );
        // println!("if the type is 32? {}",context.get_function_type(self.callee()).is_i32());
        // println!("what is the type? {}", context.get_function_type(self.callee()));
        // println!("what is the typekind? {}", context.get_function_type(self.callee()).kind());

        asm.store(f, context, value, "a0")?;
        Ok(())
    }
}

impl<'p> GenerateAsmValue<'p> for Branch {
    type Out = ();
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, _value : &Value) -> CResult<Self::Out> {
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
    fn generate_asm(&self, f : &'p mut File, context : &'p mut ProgramContext, value : &Value) -> CResult<Self::Out> {
        let asmwriter = AsmWriter::new();
        asmwriter.load(f, context, &(self.lhs()), "t0")?;
        asmwriter.load(f, context, &(self.rhs()), "t1")?;
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
                asmwriter.xor(f, "t0", "t1", "t2")?;
                asmwriter.seqz(f, "t2", "t2")?;
            },
            BinaryOp::NotEq => {
                asmwriter.xor(f, "t0", "t1", "t2")?;
                asmwriter.snez(f, "t2", "t2")?;
            },
            BinaryOp::Le => {
                asmwriter.xor(f, "t0", "t1", "t2")?;
                asmwriter.seqz(f, "t2", "t2")?;
                asmwriter.slt(f, "t0", "t1", "t1")?;
                asmwriter.or(f, "t1", "t2", "t2")?;
            },
            BinaryOp::Ge => {
                asmwriter.xor(f, "t0", "t1", "t2")?;
                asmwriter.seqz(f, "t2", "t2")?;
                asmwriter.sgt(f, "t0", "t1", "t1")?;
                asmwriter.or(f, "t1", "t2", "t2")?;
            },
            _ => ()
        }
        asmwriter.store(f, context, value, "t2")?;
        Ok(())
    }
}