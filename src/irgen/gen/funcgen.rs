use crate::ast::*;
use crate::irgen::scopes::Scopes;
use crate::irgen::func::FunctionInfo;


use koopa::ir::builder_traits::*;
use koopa::ir::{Type, FunctionData};
use crate::irgen::{IResult, GenerateIR};



impl<'ast> FuncDef {
    // build the map from name to function (id) in advance
    pub fn function_generate_forepart(&'ast self, scopes : &mut Scopes<'ast>) 
    -> IResult<()>{
        // 1. generate the function type + parameter type
        let ret_ty = self.ty.generate(scopes)?;

        // [TODO] now all the parameter is int type

        let params_ty = self
        .params
        .iter()
        .map(|_p| Type::get_i32())
        .collect();

        // 2. create a new function
        let function_data = FunctionData::new(format!("@{}", self.ident), 
        params_ty ,ret_ty);

        let func = scopes.program.new_func(function_data);

        // 3. add function to scope (global)       
        scopes.record_function(&self.ident, func);

        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncDef {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> IResult<Self::Out> {
        // 1.1 retrieve function id from global
        let func = scopes.get_global_function(&self.ident);

        // 1.2 retrieve function data from this id
        let function_data = scopes.program.func_mut(func);

        // 1.3 generate entry block 
        let entry = function_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));

        // 1.4 generate end block
        let end = function_data.dfg_mut().new_bb().basic_block(Some("%end".into()));

        // 1.5 generate current block
        let cur = function_data.dfg_mut().new_bb().basic_block(None);

        // 1.6 generate return value (if it exists)
        let return_val = match self.ty {
            FuncType::Int => {
                let alloc = function_data.dfg_mut().new_value().alloc(Type::get_i32());
                function_data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
                Some(alloc)
            },
            _ => None
        };

        
        // 1.7 the initial function information
        let function_info = FunctionInfo::new(func, entry, cur, end, return_val);

        // 1.8 add function to scope        
        scopes.cur_func = Some(function_info);
        // --- function creation is end

        // 1.9 save params
        let params = function_data.params().to_owned();

        // 1.10 kill a mutable reference
        drop(function_data);
        // --- function_data is dead

        // all we use is about "scope"
        

        scopes.function_add_block(entry);

        // add retrun allocation (if have)
        match return_val {
            Some(v) => {
                scopes.function_push_inst(v);
            },
            _ => ()
        };
        scopes.function_add_block(cur);


        scopes.add_level();
        // next level
        // 2.1 generate place for parameters

        for (param, value) in self.params.iter().zip(params) {
            let ty = scopes.get_value_type(value);
            let alloc = scopes.create_initial_variable(ty, Some(param.id.as_str()));
            let store = scopes.new_value().store(value, alloc);
            scopes.function_push_inst(store);
            scopes.add_variable_name(param.id.as_str(), alloc);
        }

        
        // 2.2 generate body
        self.block.generate(scopes)?;

        scopes.minus_level();
        // previous level

        // close function 
        scopes.close_entry(cur);
        scopes.close_function(end);


        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncType {
    type Out = Type;
    
    fn generate(&'ast self, _scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        Ok(
            match self {
                Self::Int => Type::get_i32(),
                Self::Void => Type::get_unit()
            }
        )
    }
}