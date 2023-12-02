mod expgen;
mod opgen;
mod declgen;
mod stmtgen;
use crate::ast::*;
use koopa::ir::{builder_traits::*, TypeKind};
use koopa::ir::{FunctionData, Program, Type};
use super::func::FunctionInfo;
use super::scopes::Scopes;
use super::{Result, scopes};



// Trait for generating Koopa IR for all ast component.

pub trait GenerateIR<'ast> {
    type Out;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> Result<Self::Out>;
}

impl<'ast> GenerateIR<'ast> for CompUnit {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> Result<Self::Out>{
        self.func_def.generate(scopes)?;
        Ok(())
    }
}


impl<'ast> GenerateIR<'ast> for FuncDef {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> Result<Self::Out> {
        // 1. generate the function type
        let ret_ty = self.func_type.generate(scopes)?;


        // 2. create a new function
        let mut function_data = FunctionData::new(format!("@{}", self.ident), 
        vec![],ret_ty);

        // 3. generate entry block 
        let entry = function_data.dfg_mut().new_bb().basic_block(Some("%entry".into()));

        // 4. generate end block
        let end = function_data.dfg_mut().new_bb().basic_block(Some("%end".into()));

        // 5. generate current block
        let cur = function_data.dfg_mut().new_bb().basic_block(None);

        let return_val = match self.func_type {
            FuncType::Int => {
                let alloc = function_data.dfg_mut().new_value().alloc(Type::get_i32());
                function_data.dfg_mut().set_value_name(alloc, Some("%ret".into()));
                Some(alloc)
            },
            _ => None
        };

        // 6. update function information in program Function Hashmap
        // add function in the program
        let func = scopes.program.new_func(function_data);
        

        let function_info = FunctionInfo::new(func, entry, cur, end, return_val);
        // --- function creation is end

        // 7. add function to scope        
        scopes.record_function(&self.ident, func);
        scopes.cur_func = Some(function_info);


        
        // 8. (scope and program is ready)
        
        // program: create a new function + get its function id 
        // scopes: set function table + current function
        scopes.function_add_block(entry);

        // add retrun allocation (if have)
        match return_val {
            Some(v) => {
                scopes.function_push_inst(v);
            },
            _ => ()
        };
        scopes.function_add_block(cur);


        //[TO BE ADDED]scopes.add_level();
        // next level
        self.block.generate(scopes)?;

        //[TO BE ADDED]scopes.minus_level();

        // close function 
        scopes.close_entry(cur);
        scopes.close_function(end);


        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for FuncType {
    type Out = Type;
    
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
        Ok(
            match self {
                Self::Int => Type::get_i32()
            }
        )
    }
}


impl<'ast> GenerateIR<'ast> for Block {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
        scopes.add_level();
        for item in self.items.iter() {
            item.generate(scopes)?;
        }
        scopes.minus_level();
        //self.stmt.generate(program, context)?;
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for BlockItem {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
        match self {
            BlockItem::Decl(decl) => decl.generate(scopes)?,
            BlockItem::Stmt(stmt) => stmt.generate(scopes)?
        }
        Ok(())
    }
}



