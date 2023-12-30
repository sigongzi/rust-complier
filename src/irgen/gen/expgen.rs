use crate::ast::*;


use koopa::ir::builder_traits::*;
use koopa::ir::{Value, Type, BinaryOp};
use crate::irgen::scopes::{Scopes};
use crate::irgen::{IResult, GenerateIR};
use crate::irgen::IRError;
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
    pub fn into_int(self, scopes: &mut Scopes) -> IResult<Value> {
        match self {
            // there is an error when expresult is void
            Self::Void => Err(IRError::VoidValue),
            // unwrap the value
            Self::Int(v) => Ok(v),
            Self::IntPtr(v) => {
                let load = scopes.new_value().load(v);
                scopes.function_push_inst(load);
                Ok(load)
            }
        }
    }

    pub fn into_ptr(self) -> IResult<Value>{
        match self {
            Self::IntPtr(v) => Ok(v),
            _ => Err(IRError::NotMemory)
        }
    }
}

impl<'ast> GenerateIR<'ast> for LVal {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes) 
        -> IResult<Self::Out> {

            
            let var = scopes.retrieve_val(&self.id).ok_or(IRError::UndefinedLVal(self.id.clone()))?;
            Ok(ExpResult::IntPtr(var))
    }
}

impl<'ast> GenerateIR<'ast> for Exp {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        self.lor.generate(scopes)
    }
}


impl<'ast> GenerateIR<'ast> for ConstExp {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        self.exp.generate(scopes)
    }
}


impl<'ast> GenerateIR<'ast> for PrimaryExp {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        match self {
            Self::Ausdruck(exp) => exp.generate(scopes),
            // return primary number
            Self::Number(num) => {
                let res = scopes.new_value().integer(*num);
                Ok(ExpResult::Int(res))
            },
            Self::LVal(lval) => {
                lval.generate(scopes)
            } 
        }
    }    
}


impl<'ast> GenerateIR<'ast> for UnaryExp {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>)
        -> IResult<Self::Out> {
        match self {
            Self::PrimaryAusdruck(p) => p.generate(scopes),
            Self::UnaryAusdruck(op, exp) => {
                
                let exp_result = exp.generate(scopes)?;

                let zero = scopes.new_value().integer(0);
                
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
                        let rhs = exp_result.into_int(scopes)?;
                        let res = scopes.new_value().binary(binary_op, zero, rhs);
                        scopes.function_push_inst(res);
                        Ok(ExpResult::Int(res))
                    },
                }
            },
            Self::Call(call) => call.generate(scopes)
        }
        
    }
}

impl<'ast> GenerateIR<'ast> for FuncCall {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {

        let callee = scopes.get_global_function(self.id.as_str()); 
        let args  = self.params
        .iter()
        .map(|p| {
                let res = p.generate(scopes)?.into_int(scopes)?;
                Ok(res)
            }
        )
        .collect::<IResult<Vec<_> > >()?;

        let call = scopes.new_value().call(callee, args);
        scopes.function_push_inst(call);
        Ok(ExpResult::Int(call))
    }
}

// generate trait for all binary expression
macro_rules! implement_trait_for_binary_expression {
    ($trait_name:ident for $(($prev:ident, $cur:ident)),+) => {
        $(
            paste! {
impl<'ast> $trait_name<'ast> for [<$cur Exp>]  {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        match self {
            [<$cur Exp>]::[<$prev Ausdruck>](a) => a.generate(scopes),
            [<$cur Exp>]::[<$cur Ausdruck>](lhs, op, rhs) => {
                
                let lhs_res = lhs.generate(scopes)?.into_int(scopes)?;
                let rhs_res = rhs.generate(scopes)?.into_int(scopes)?;
                let binary_op = op.select_binary_op();
                let res = scopes.new_value().binary(binary_op, 
                    lhs_res, 
                    rhs_res);
                scopes.function_push_inst(res);
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
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        match self {
            [<$cur Exp>]::[<$prev Ausdruck>](a) => a.generate(scopes),
            [<$cur Exp>]::[<$cur Ausdruck>](lhs, rhs) => {
                let zero = scopes.new_value().integer(0);
                let one = scopes.new_value().integer(1);
                let binary_op = (if stringify!($cur) == "LAnd" {
                    BinaryOp::NotEq
                } else {
                    BinaryOp::Eq
                });

                // generate a initial variable(global in this function) to store the result
                // for LAnd, lhs != 0 failed, the result is 0, and we go to end directly
                // for LOr,  lhs == 0 failed, the result is 1, and we go to end directly

                let final_result = scopes.create_initial_variable(Type::get_i32(), None);

                let store = if stringify!($cur) == "LAnd" {
                    scopes.new_value().store(zero, final_result)
                } else {
                    scopes.new_value().store(one, final_result)
                };

                scopes.function_push_inst(store);

                let lhs_result = lhs.generate(scopes)?.into_int(scopes)?;

                // 1. calculate lhs result
                let lhs_comp_result = scopes.new_value().binary(binary_op, zero, lhs_result);

                // 2.add compare result in current block
                scopes.function_push_inst(lhs_comp_result);

                // For LAnd and LOr
                // if lhs is true, we need to test rhs
                // or else we go to end block directly

                let next_block = scopes.create_new_block(Some(concat!("%", stringify!($cur), "_next").to_string()));
                let end_block = scopes.create_new_block(Some(concat!("%", stringify!($cur),"_end").to_string()));

                let br = scopes.new_value().branch(lhs_comp_result, next_block, end_block);
                
                // 3.add branch to the current block
                scopes.function_push_inst(br);
                    
                // 4. add next_block, now next_block is current block
                scopes.function_add_block(next_block);
                

                // 5. calculate right hand side and add instruction in next_block
                let rhs_result = rhs.generate(scopes)?.into_int(scopes)?;

                let rhs_comp_result = scopes.new_value().binary(BinaryOp::NotEq, zero, rhs_result);

                scopes.function_push_inst(rhs_comp_result);

                // 6. store the right hand result in final result

                let store_rhs = scopes.new_value().store(rhs_comp_result, final_result);

                scopes.function_push_inst(store_rhs);

                // 7. from next_block jump to the end block
                let jump_to_end = scopes.new_value().jump(end_block);

                scopes.function_push_inst(jump_to_end);

                // 8. add end_block as current block
                scopes.function_add_block(end_block);

                // 9. load the final result
                let load_result = scopes.new_value().load(final_result);

                scopes.function_push_inst(load_result);
                Ok(ExpResult::Int(load_result))
                /*
                let mut zero: Option<Value> = None;
                let mut one: Option<Value> = None;
                let mut final_result: Option<Value> = None;
                let mut binary_op: Option<BinaryOp> = None;
                {
                    let local_handler = LocalHandler::new(program, scopes);

                    // zero and one
                    zero = Some(local_handler.new_value().integer(0));
                    one = Some(local_handler.new_value().integer(1));


                    // choosee binary operator for land
                    binary_op = Some(if stringify!($cur) == "LAnd" {
                        BinaryOp::NotEq
                    } else {
                        BinaryOp::Eq
                    });

                    // generate a initial variable(global in this function) to store the result
                    // for LAnd, lhs != 0 failed, the result is 0, and we go to end directly
                    // for LOr,  lhs == 0 failed, the result is 1, and we go to end directly

                    final_result = Some(local_handler.create_initial_variable(Type::get_i32(), None));

                    let store = if stringify!($cur) == "LAnd" {
                        local_handler.new_value().store(zero.unwrap(), final_result.unwrap())
                    } else {
                        local_handler.new_value().store(one.unwrap(), final_result.unwrap())
                    };
                    // 1.push the initial result in current block
                    local_handler.push_inst(program, store);
                }

                
                let lhs_result = lhs.generate(program, scopes)?;
                
                {
                    let lhs_comp_result = cur_func!(scopes).new_value(program).binary(binary_op, zero, lhs_result);

                    // 2.add compare result in current block
                    cur_func!(scopes).push_inst(program, lhs_comp_result);


                    // For LAnd and LOr
                    // if lhs is true, we need to test rhs
                    // or else we go to end block directly

                    let next_block = cur_func!(scopes).create_new_block(program, Some(concat!("%", stringify!($cur), "_next").to_string()));
                    let end_block = cur_func!(scopes).create_new_block(program, Some(concat!("%", stringify!($cur),"_end").to_string()));

                    let br = cur_func!(scopes).new_value(program).branch(lhs_comp_result, next_block, end_block);
                    // 3.add branch to the current block
                    cur_func!(scopes).push_inst(program, br);
                    
                    // 4. add next_block, now next_block is current block
                    cur_func!(scopes).add_new_block(program, next_block);
                }
                // 5. calculate right hand side and add instruction in next_block
                let rhs_result = rhs.generate(program, scopes)?;

                {
                    let rhs_comp_result = cur_func!(scopes).new_value(program).binary(binary_op, zero, rhs_result);

                    cur_func!(scopes).push_inst(program, rhs_comp_result);

                    // 6. store the right hand result in final result

                    let store_rhs = cur_func!(scopes).new_value(program).store(rhs_comp_result, final_result);

                    cur_func!(scopes).push_inst(program, store_rhs);

                    // 7. from next_block jump to the end block
                    let jump_to_end = cur_func!(scopes).new_value(program).jump(end_block);

                    cur_func!(scopes).push_inst(program, jump_to_end);

                    // 8. add end_block as current block
                    cur_func!(scopes).add_new_block(program, end_block);
                }
                /*// 9. load the final result
                let load_result = cur_func!(scopes).new_value(program).load(final_result);

                cur_func!(scopes).push_inst(program, load_result);*/
                Ok(ExpResult::IntPtr(final_result.unwrap()))
            */
            }
            
        }
        
    }
}
            }
        )+
    };
}

implement_trait_for_logic_expression!(GenerateIR for (Eq, LAnd), (LAnd, LOr));