use crate::ast::*;

use koopa::ir::builder_traits::*;
use koopa::ir::Program;
use koopa::ir::Value;
use crate::irgen::context::Context;
use crate::irgen::{Result, GenerateIR};
use crate::irgen::IRError;
use koopa::ir::BinaryOp;
use paste::paste;
use super::opgen::SelectBinaryOp;

/// pass the result of a exp
/// Value: a structrue in koopa IR
/// We have no variable here
/// Void may not be constructed
#[allow(dead_code)]
pub enum ExpResult {
    Void,
    Int(Value)
}

impl ExpResult {
    pub fn into_int(self) -> Result<Value> {
        match self {
            // there is an error when expresult is void
            Self::Void => Err(IRError::VoidValue),
            // unwrap the value
            Self::Int(v) => Ok(v)
        }
    }
}

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.lor.generate(program, context)
    }
}

impl<'ast> GenerateIR<'ast> for PrimaryExp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut crate::irgen::context::Context) 
        -> Result<Self::Out> {
        match self {
            Self::Ausdruck(exp) => exp.generate(program, context),
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
            Self::PrimaryAusdruck(p) => p.generate(program, context),
            Self::UnaryAusdruck(op, exp) => {
                let exp_result = exp.generate(program, context)?
                .into_int()?;

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


// generate trait for all binary expression
macro_rules! implement_trait_for_binary_expression {
    ($trait_name:ident for $(($prev:ident, $cur:ident)),+) => {
        $(
            paste! {
                impl<'ast> $trait_name<'ast> for [<$cur Exp>]  {
                    type Out = ExpResult;
                    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
                        -> Result<Self::Out> {
                        match self {
                            [<$cur Exp>]::[<$prev Ausdruck>](a) => a.generate(program, context),
                            [<$cur Exp>]::[<$cur Ausdruck>](lhs, op, rhs) => {
                                let lhs_res = lhs.generate(program, context)?
                                .into_int()?;
                                let rhs_res = rhs.generate(program, context)?
                                .into_int()?;
                                let binary_op = op.select_binary_op();
                                let cur_func = context.get_current_func();
                                let res = cur_func.new_value(program).binary(binary_op, lhs_res, rhs_res);
                                cur_func.push_inst_to_entry(program, res);
                                Ok(ExpResult::Int(res))
                            }
                        }
                    }
                }
            }
        )+
    };
}

implement_trait_for_binary_expression!(GenerateIR 
    for (Unary,Mul), 
        (Mul, Add),
        (Add, Rel),
        (Rel, Eq));


macro_rules! implement_trait_for_logic_expression {
    ($trait_name:ident for $(($prev:ident, $cur:ident)),+) => {
        $(
            paste! {
                impl<'ast> $trait_name<'ast> for [<$cur Exp>]  {
                    type Out = ExpResult;
                    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
                        -> Result<Self::Out> {
                        match self {
                            [<$cur Exp>]::[<$prev Ausdruck>](a) => a.generate(program, context),
                            [<$cur Exp>]::[<$cur Ausdruck>](lhs, rhs) => {
                                Ok(ExpResult::Void)
                            }
                        }
                    }
                }
            }
        )+
    };
}

implement_trait_for_logic_expression!(GenerateIR for (Eq, LAnd), (LAnd, LOr));