use crate::ast::*;


use koopa::ir::builder_traits::*;
use koopa::ir::{Value, Program, Type};
use crate::irgen::context::{Context};
use crate::cur_func;
use crate::irgen::{Result, GenerateIR};
use crate::irgen::IRError;
use koopa::ir::BinaryOp;
use paste::paste;
use super::opgen::{SelectBinaryOp};

/// pass the result of a exp
/// Value: a structrue in koopa IR
/// We have no variable here
/// Void may not be constructed
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ExpResult {
    Void,
    Int(Value),
    IntPtr(Value)
}

impl ExpResult {
    pub fn into_int(self,program: &mut Program,context: &mut Context) -> Result<Value> {
        match self {
            // there is an error when expresult is void
            Self::Void => Err(IRError::VoidValue),
            // unwrap the value
            Self::Int(v) => Ok(v),
            Self::IntPtr(v) => {
                let load = cur_func!(context).new_value(program).load(v);
                cur_func!(context).push_inst(program, load);
                Ok(load)
            }
        }
    }

    pub fn into_ptr(self) -> Result<Value>{
        match self {
            Self::IntPtr(v) => Ok(v),
            _ => Err(IRError::NotMemory)
        }
    }
}

impl<'ast> GenerateIR<'ast> for LVal {
    type Out = ExpResult;
    fn generate(&'ast self, _program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
            let var = cur_func!(context).require_val(&self.id).ok_or(IRError::UndefinedLVal((&self.id).to_string()))?;
            Ok(ExpResult::IntPtr(var))
    }
}

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.lor.generate(program, context)
    }
}


impl<'ast> GenerateIR<'ast> for ConstExp {
    type Out = ExpResult;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.exp.generate(program, context)
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
            ),
            Self::LVal(lval) => {
                lval.generate(program, context)
            } 
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
                
                let exp_result = exp.generate(program, context)?;

                let zero = cur_func!(context).new_value(program).integer(0);
                
                match op {
                    // +: Do nothing
                    UnaryOp::Positive => Ok(exp_result),
                    // -: sub 0, %prev_exp (0 - %prev_exp)
                    // !: eq 0, %prev_exp (0 == %prev_exp)
                    UnaryOp::Negative | UnaryOp::LNot=> {
                        let binary_op = match op {
                            UnaryOp::Negative => BinaryOp::Sub, 
                            UnaryOp::LNot => BinaryOp::Eq,
                            _ => unreachable!()
                        };
                        let rhs = exp_result.into_int(program, context)?;
                        let res = cur_func!(context).new_value(program).binary(binary_op, zero, rhs);
                        cur_func!(context).push_inst(program, res);
                        Ok(ExpResult::Int(res))
                    },
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
                let lhs_res = lhs.generate(program, context)?.into_int(program, context)?;
                let rhs_res = rhs.generate(program, context)?.into_int(program, context)?;
                let binary_op = op.select_binary_op();
                let res = cur_func!(context).new_value(program).binary(binary_op, 
                    lhs_res, 
                    rhs_res);
                cur_func!(context).push_inst(program, res);
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

                // zero and one
                let zero = cur_func!(context).new_value(program).integer(0);
                let one = cur_func!(context).new_value(program).integer(1);


                // choosee binary operator for land
                let binary_op = if stringify!($cur) == "LAnd" {
                    BinaryOp::NotEq
                } else {
                    BinaryOp::Eq
                };

                // generate a initial variable(global in this function) to store the result
                // for LAnd, lhs != 0 failed, the result is 0, and we go to end directly
                // for LOr,  lhs == 0 failed, the result is 1, and we go to end directly

                let final_result = cur_func!(context).create_initial_variable(program, Type::get_i32(), None);

                let store = if stringify!($cur) == "LAnd" {
                    cur_func!(context).new_value(program).store(zero, final_result)
                } else {
                    cur_func!(context).new_value(program).store(one, final_result)
                };
                // 1.push the initial result in current block
                cur_func!(context).push_inst(program, store);

                let lhs_result = lhs.generate(program, context)?.into_int(program, context)?;
                let lhs_comp_result = cur_func!(context).new_value(program).binary(binary_op, zero, lhs_result);

                // 2.add compare result in current block
                cur_func!(context).push_inst(program, lhs_comp_result);


                // For LAnd and LOr
                // if lhs is true, we need to test rhs
                // or else we go to end block directly

                let next_block = cur_func!(context).create_new_block(program, Some(concat!("%", stringify!($cur), "_next").to_string()));
                let end_block = cur_func!(context).create_new_block(program, Some(concat!("%", stringify!($cur),"_end").to_string()));

                let br = cur_func!(context).new_value(program).branch(lhs_comp_result, next_block, end_block);
                // 3.add branch to the current block
                cur_func!(context).push_inst(program, br);
                
                // 4. add next_block, now next_block is current block
                cur_func!(context).add_new_block(program, next_block);

                // 5. calculate right hand side and add instruction in next_block
                let rhs_result = rhs.generate(program, context)?.into_int(program, context)?;
                let rhs_comp_result = cur_func!(context).new_value(program).binary(binary_op, zero, rhs_result);

                cur_func!(context).push_inst(program, rhs_comp_result);

                // 6. store the right hand result in final result

                let store_rhs = cur_func!(context).new_value(program).store(rhs_comp_result, final_result);

                cur_func!(context).push_inst(program, store_rhs);

                // 7. from next_block jump to the end block
                let jump_to_end = cur_func!(context).new_value(program).jump(end_block);

                cur_func!(context).push_inst(program, jump_to_end);

                // 8. add end_block as current block
                cur_func!(context).add_new_block(program, end_block);

                // 9. load the final result
                let load_result = cur_func!(context).new_value(program).load(final_result);

                cur_func!(context).push_inst(program, load_result);

                Ok(ExpResult::IntPtr(load_result))
            }
        }
    }
}
            }
        )+
    };
}

implement_trait_for_logic_expression!(GenerateIR for (Eq, LAnd), (LAnd, LOr));