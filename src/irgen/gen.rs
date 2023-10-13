mod expgen;
mod opgen;
use crate::{ast::*, cur_func};
use koopa::ir::{builder_traits::*, TypeKind};
use koopa::ir::{FunctionData, Program, Type};
use super::context::Context;
use super::Result;
use super::func::FunctionHandler;
use expgen::ExpResult;

// Trait for generating Koopa IR for all ast component.

pub trait GenerateIR<'ast> {
    type Out;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
    -> Result<Self::Out>;
}

impl<'ast> GenerateIR<'ast> for CompUnit {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
    -> Result<Self::Out>{
        self.func_def.generate(program, context)?;
        Ok(())
    }
}


impl<'ast> GenerateIR<'ast> for FuncDef {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
    -> Result<Self::Out> {
        // generate the function type
        let ret_ty = self.func_type.generate(program, context)?;


        // create a new function
        let mut function_data = FunctionData::new(format!("@{}", self.ident), 
        vec![],ret_ty);

        // generate entry block 
        let entry = function_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));
        let end = function_data.dfg_mut().new_bb().basic_block(Some("%end".into()));
        let cur = function_data.dfg_mut().new_bb().basic_block(None);

        function_data.layout_mut().bbs_mut().extend([entry, cur, end]);

        


        // update function information in program Function Hashmap
        let func = program.new_func(function_data);

        let function_handler = FunctionHandler::new(func, entry, cur, end);
        

        

        // insert function handler in to context as current function
        context.current_func = Some(function_handler);
        
        

        

        // alloc return in entry block(if have)
        match cur_func!(context).get_function_kind(program) {
            &TypeKind::Int32 => {
                //println!("return type is i32");
                let ret_val =  cur_func!(context).create_initial_variable(program, Type::get_i32(), Some("ret".into()));

                let load = cur_func!(context).new_value(program).load(ret_val);
                let ret = cur_func!(context).new_value(program).ret(Some(load));
                
                cur_func!(context).set_ret_val(ret_val);

                cur_func!(context).push_inst_to(program, end, load);
                cur_func!(context).push_inst_to(program, end, ret);

            },
            _ => {
                //println!("return type is not i32");
                let ret = cur_func!(context).new_value(program).ret(None);

                cur_func!(context).push_inst_to(program, end, ret);
            }
        }; 

        self.block.generate(program, context)?;

        // TODO: temporary method to prevent empty entry block
        let jump = cur_func!(context).new_value(program).jump(cur);
        cur_func!(context).push_inst_to(program, entry, jump);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncType {
    type Out = Type;
    fn generate(&'ast self, _program: &mut Program, _context : &mut Context) 
        -> Result<Self::Out> {
        Ok(
        match self {
            Self::Int => Type::get_i32()
        })
    }
}


impl<'ast> GenerateIR<'ast> for Block {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        self.stmt.generate(program, context)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        // get expression result for return 
        let exp_result = self.exp.generate(program, context)?;

        let cur_func = context.get_current_func();
        

        // generate return command
        let v = match exp_result {
            ExpResult::Void => None,
            ExpResult::Int(v) => Some(v),
            ExpResult::IntPtr(v) => Some(v)
        };
        if let Some(val) = v {
            let store = cur_func.new_value(program).store(val, 
            cur_func.get_ret_value());
            cur_func.push_inst(program, store);
            
        }
        let jump = cur_func.new_value(program).jump(cur_func.get_end());
        cur_func.push_inst(program, jump);
        Ok(())
    }
}


