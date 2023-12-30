use crate::ast::*;

use koopa::ir::builder_traits::*;
use koopa::ir::{Program, Type};
use crate::irgen::scopes::Scopes;
use crate::irgen::{IResult, GenerateIR};



use super::expgen::{*};

impl<'ast> GenerateIR<'ast> for Decl{
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        match self {
            Decl::Const(c) => c.generate(scopes),
            Decl::Var(v) => v.generate(scopes)
        }
    }
}

impl<'ast> GenerateIR<'ast> for VarDecl {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
            for def in self.defs.iter() {
                def.generate(scopes)?;
            }
            Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for VarDef {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        
        let tar = scopes.create_initial_variable(Type::get_i32(), Some(&self.id));
        scopes.add_variable_name(&self.id, tar);
        match &self.init_val {
            Some(e) => {
                let res = e.generate(scopes)?.into_int(scopes)?;
                let store = scopes.new_value().store(res, tar);
                scopes.function_push_inst(store);
            },
            None => ()
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for InitVal {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        self.exp.generate(scopes)
    }
}
impl<'ast> GenerateIR<'ast> for ConstDecl {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
            for def in self.defs.iter() {
                def.generate(scopes)?;
            }
            Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for ConstDef {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        // a temporary method, treat const as variable
        let tar = scopes.create_initial_variable(Type::get_i32(), Some(&self.id));
        scopes.add_variable_name(&self.id, tar);
        let res = self.init_val.generate(scopes)?.into_int(scopes)?;
        let store = scopes.new_value().store(res, tar);

        scopes.function_push_inst(store);
        Ok(())
    }
}

// generate IR for const val
impl<'ast> GenerateIR<'ast> for ConstInitVal {
    type Out = ExpResult;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        self.exp.generate(scopes)
    }
}
