use crate::ast::*;


use koopa::ir::builder_traits::*;
use koopa::ir::{Program};
use crate::irgen::context::{Context};
use crate::cur_func;
use crate::irgen::{Result, GenerateIR};





impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
        // get expression result for return 
        match self {
            Self::Return(r) => r.generate(program, context),
            Self::Assign(a) => a.generate(program, context)
        }
    }
}

impl<'ast> GenerateIR<'ast> for Return {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
            let res = self.exp.generate(program, context)?.into_int(program, context)?;

            let dest = cur_func!(context).get_ret_value();
            let store = cur_func!(context).new_value(program).store(res, dest);
            let jump = cur_func!(context).new_value(program).jump(cur_func!(context).get_end());
            cur_func!(context).push_inst(program, store);
            cur_func!(context).push_inst(program, jump);
            Ok(())
        
    }
}

impl<'ast> GenerateIR<'ast> for Assign {
    type Out = ();
    fn generate(&'ast self, program: &mut Program, context : &mut Context) 
        -> Result<Self::Out> {
            let res = self.exp.generate(program, context)?.into_int(program, context)?; 
            let dest = self.lval.generate(program, context)?.into_ptr()?;
            let store = cur_func!(context).new_value(program).store(res, dest);
            cur_func!(context).push_inst(program, store);
            Ok(())
    }
}
