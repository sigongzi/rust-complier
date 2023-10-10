mod expgen;
use crate::ast::*;
use koopa::ir::builder_traits::*;
use koopa::ir::{FunctionData, Program, Type};
use super::context::Context;
use super::Result;
use super::func::FunctionHandler;

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

        // add the entry block into function data
        function_data.layout_mut().bbs_mut().extend([entry]);

        // update function information in program Function Hashmap
        let func = program.new_func(function_data);

        let mut function_handler = FunctionHandler::new(func, entry);

        context.current_func = Some(function_handler);
        // insert function handler in to context as current function
        self.block.generate(program, context)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncType {
    type Out = Type;
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
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
        let exp = self.exp.generate(program, context);    
        let cur_func = context.get_current_func();
        
        let num = cur_func.new_value(program).integer(0);
        let ret = cur_func.new_value(program).ret(Some(num));

        cur_func.push_inst_to_entry(program, ret);
        Ok(())
    }
}


