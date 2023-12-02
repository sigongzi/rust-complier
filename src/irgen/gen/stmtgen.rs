use crate::ast::*;
use crate::irgen::scopes::Scopes;


use koopa::back::generator;
use koopa::ir::builder_traits::*;
use koopa::ir::{Program};
use crate::irgen::{Result, GenerateIR};

use super::expgen::ExpResult;





impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
        // get expression result for return 
        match self {
            Self::Return(r) => r.generate(scopes),
            Self::Assign(a) => a.generate(scopes),
            Self::Block(b) => b.generate(scopes),
            Self::ExpStmt(e) => e.generate(scopes)
        }
    }
}

impl<'ast> GenerateIR<'ast> for Return {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
            match &self.exp {
                Some(e) => {
                    let res = e.generate(scopes)?.into_int(scopes)?;
                    let dest = scopes.get_ret().unwrap();
                    let store = scopes.new_value().store(res, dest);
                    scopes.function_push_inst(store);
                },
                None => ()
            };
            let end_block = scopes.get_end();
            let jump = scopes.new_value().jump(end_block);
            scopes.function_push_inst(jump);
            Ok(())
        
    }
}

impl<'ast> GenerateIR<'ast> for Assign {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
            let res = self.exp.generate(scopes)?.into_int(scopes)?; 
            let dest = self.lval.generate(scopes)?.into_ptr()?;
            let store = scopes.new_value().store(res, dest);
            scopes.function_push_inst(store);
            Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ExpStmt {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> Result<Self::Out> {
        if let Some(e) = &self.exp {
            e.generate(scopes)?;
        }
        Ok(())
    }
}