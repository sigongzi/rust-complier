use crate::ast::*;

use koopa::ir::builder_traits::*;
use koopa::ir::Program;
use koopa::ir::Value;
use crate::irgen::context::Context;
use crate::irgen::{Result, GenerateIR};

use koopa::ir::BinaryOp;

/// pass the result of a exp
/// Value: a structrue in koopa IR
/// We have no variable here
pub enum ExpResult {
    Int(Value)
}

impl ExpResult {
    pub fn into_int(self, _program: &mut Program, _context: &mut Context) -> Result<Value> {
        match self {
            // there is an error when expresult is void
            // Self::Void => Err(IRError::VoidValue),
            // unwrap the value
            Self::Int(v) => Ok(v)
        }
    }
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
            // return primary number
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
        match self {
            Self::Primary(p) => p.generate(program, context),
            Self::Unary(op, exp) => {
                let exp_result = exp.generate(program, context)?
                .into_int(program, context)?;

                let cur_func = context.get_current_func();
                let zero = cur_func.new_value(program).integer(0);
                match op {
                    // +: Do nothing
                    UnaryOp::Positive => Ok(ExpResult::Int(exp_result)),
                    // -: sub 0, %prev_exp (0 - %prev_exp)
                    UnaryOp::Negative => {

                        let res = cur_func.new_value(program).binary(BinaryOp::Sub, zero, exp_result);
                        cur_func.push_inst_to_entry(program, res);
                        Ok(ExpResult::Int(res))
                    },
                    // !: eq 0, %prev_exp (0 == %prev_exp)
                    UnaryOp::LNot => {
                        let res = cur_func.new_value(program).binary(BinaryOp::Eq, zero, exp_result);
                        cur_func.push_inst_to_entry(program, res);
                        Ok(ExpResult::Int(res))
                    }
                }
            }
        }
    }
}


