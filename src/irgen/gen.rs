mod expgen;
mod opgen;
mod declgen;
mod stmtgen;
mod funcgen;

use crate::ast::*;

use koopa::ir::{Type};

use super::scopes::Scopes;
use super::{IResult};



// Trait for generating Koopa IR for all ast component.

pub trait GenerateIR<'ast> {
    type Out;
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> IResult<Self::Out>;
}

impl<'ast> GenerateIR<'ast> for CompUnit {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
    -> IResult<Self::Out>{
        scopes.decl_func("getint", vec![], Type::get_i32());
        scopes.decl_func("getch", vec![], Type::get_i32());
        scopes.decl_func("getarray", vec![Type::get_pointer(Type::get_i32())], Type::get_i32());
        scopes.decl_func("putint", vec![Type::get_i32()], Type::get_unit());
        scopes.decl_func("putch", vec![Type::get_i32()], Type::get_unit());

        scopes.decl_func("putarray", vec![Type::get_i32(), Type::get_pointer(Type::get_i32())], Type::get_unit());

        scopes.decl_func("starttime", vec![], Type::get_unit());

        scopes.decl_func("stoptime", vec![], Type::get_unit());
        
        for func in &self.func_def {
            func.function_generate_forepart(scopes)?;
        }
        for func in &self.func_def {
            func.generate(scopes)?;
        }
        Ok(())
    }
}





impl<'ast> GenerateIR<'ast> for Block {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        scopes.add_level();
        for item in self.items.iter() {
            item.generate(scopes)?;
        }
        scopes.minus_level();
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for BlockItem {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        match self {
            BlockItem::Decl(decl) => decl.generate(scopes)?,
            BlockItem::Stmt(stmt) => stmt.generate(scopes)?
        }
        Ok(())
    }
}



