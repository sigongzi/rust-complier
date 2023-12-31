mod expgen;
mod opgen;
mod declgen;
mod stmtgen;
mod funcgen;


pub use expgen::ExpResult;
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
        println!("the length of compdefs {}",self.comp_defs.len());
        self.comp_defs
        .iter()
        .try_for_each(|s| match s {
                CompDef::FuncDef(f) => {
                    //println!("generate function {}",f.ident);
                    f.function_generate_forepart(scopes)
                },
                _ => Ok(())
            }
        )?;
        // global level
        scopes.add_level();

        // generate all variable first
        let _ = self.comp_defs
        .iter()
        .try_for_each(|s| match s {
            CompDef::Decl(d) => {
                d.generate(scopes)
            },
            _ => Ok(())
        })?;

        self.comp_defs
        .iter()
        .try_for_each(|s| match s {
            CompDef::FuncDef(f) => {
                f.generate(scopes)
            },
            _ => Ok(())
        })?;

        scopes.minus_level();
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



