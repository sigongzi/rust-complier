use crate::ast::*;
use crate::irgen::constcalc::PreCalculate;

use koopa::ir::builder_traits::*;
use koopa::ir::{Type};
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
        let value = match scopes.is_global() {
            true => {
                let alloc_val = 
                match &self.init_val {
                    Some(v) => {
                        let res = v.exp.calculate(scopes)?;
                        scopes.program.new_value().integer(res)
                        
                    },
                    None => scopes.program.new_value().zero_init(Type::get_i32())
                };
                let v = scopes
                .program
                .new_value()
                .global_alloc(
                    alloc_val
                );
                scopes.program.set_value_name(v, Some(format!("@{}", self.id)));
                v
            },
            false => {
                let tar = scopes.create_initial_variable(Type::get_i32(), Some(&self.id));

                match &self.init_val {
                    Some(e) => {
                        let res = e.generate(scopes)?.into_int(scopes)?;
                        let store = scopes.new_value().store(res, tar);
                        scopes.function_push_inst(store);
                    },
                    None => ()
                }
                tar
            }
        };
        
        scopes.add_variable_name(&self.id, value);
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
        let v = self.init_val.generate(scopes)?;
        scopes.add_const_value_name(self.id.as_str(), v);
        Ok(())
    }
}

// generate IR for const val
impl<'ast> GenerateIR<'ast> for ConstInitVal {
    type Out = i32;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        self.exp.generate(scopes)
    }
}
