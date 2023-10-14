use crate::ast::*;

use koopa::ir::builder_traits::*;
use koopa::ir::{Value, Program, Type};
use crate::irgen::context::Context;
use crate::cur_func;
use crate::irgen::{Result, GenerateIR};
use crate::irgen::IRError;
use koopa::ir::BinaryOp;
use paste::paste;
use super::expgen::{*};

impl<'ast> GenerateIR<'ast> for Decl{
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        match self {
            Decl::Const(c) => c.generate(program, context),
            Decl::Var(v) => v.generate(program, context)
        }
    }
}

impl<'ast> GenerateIR<'ast> for VarDecl {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
            for def in self.defs.iter() {
                def.generate(program, context)?;
            }
            Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for VarDef {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        let tar = cur_func!(context).create_initial_variable(program, Type::get_i32(), Some(&self.id));
        cur_func!(context).add_const_val(self.id.to_string(), tar);
        match &self.init_val {
            Some(e) => {
                let res = e.generate(program, context)?.into_int(program, context)?;
                let store = cur_func!(context).new_value(program).store(res, tar);

                cur_func!(context).push_inst(program, store);
            },
            None => ()
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for InitVal {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}
impl<'ast> GenerateIR<'ast> for ConstDecl {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
            for def in self.defs.iter() {
                def.generate(program, context)?;
            }
            Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ConstDef {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        // a temporary method, treat const as variable
        let tar = cur_func!(context).create_initial_variable(program, Type::get_i32(), Some(&self.id));
        cur_func!(context).add_const_val(self.id.to_string(), tar);
        let res = self.init_val.generate(program, context)?.into_int(program, context)?;
        let store = cur_func!(context).new_value(program).store(res, tar);

        cur_func!(context).push_inst(program, store);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ConstInitVal {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.exp.generate(program, context)
    }
}
