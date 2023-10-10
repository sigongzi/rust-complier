use crate::ast::*;
use crate::irgen::IRError;
use koopa::ir::builder_traits::*;
use koopa::ir::{FunctionData, Program, Type};
use koopa::ir::Value as ExpId;
use crate::irgen::context::Context;
use crate::irgen::{Result, GenerateIR};
use crate::irgen::func::FunctionHandler;

pub enum ExpResult {
    Void,
    Int(ExpId)
}

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.unary.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for PrimaryExp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut crate::irgen::context::Context) 
        -> Result<Self::Out> {
        match self {
            Self::Exp(exp) => exp.generate(program, context),
            Self::Number(num) => Ok(
                ExpResult::Int(
                    context.get_current_func().new_value(program).integer(*num)
                )
            )
        }
    }    
}

impl<'ast> GenerateIR<'ast> for UnaryExp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut crate::irgen::context::Context) 
        -> Result<Self::Out> {
        Ok(ExpResult::Void)
    }
}


