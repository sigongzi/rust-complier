use crate::ast::*;
use crate::irgen::scopes::Scopes;


use koopa::back::generator;
use koopa::ir::builder_traits::*;
use koopa::ir::{Program};
use crate::irgen::{IResult, GenerateIR};

use super::expgen::ExpResult;





impl<'ast> GenerateIR<'ast> for Stmt {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        // get expression result for return 
        match self {
            Self::Return(r) => r.generate(scopes),
            Self::Assign(a) => a.generate(scopes),
            Self::Block(b) => b.generate(scopes),
            Self::ExpStmt(e) => e.generate(scopes),
            Self::If(f) => f.generate(scopes),
            Self::Break(b) => b.generate(scopes),
            Self::Continue(c) => c.generate(scopes),
            Self::While(w) => w.generate(scopes)
        }
    }
}

impl<'ast> GenerateIR<'ast> for Return {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
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

            let next_block = scopes.create_new_block(None);
            scopes.function_add_block(next_block);
            Ok(())
        
    }
}

impl<'ast> GenerateIR<'ast> for Assign {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
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
        -> IResult<Self::Out> {
        if let Some(e) = &self.exp {
            e.generate(scopes)?;
        }
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for If {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        let cond_result = self
        .condition
        .generate(scopes)?.into_int(scopes)?;
        
        let then_block = scopes.create_new_block(Some("%If_then".into()));
        
        let end_block = scopes.create_new_block(Some("%If_end".into()));
        match &self.else_then {
            Some(else_then) => {
                let else_block = scopes.create_new_block(Some("%If_else".into()));

                // 1. generate branch instruction
                let br = scopes.new_value().branch(cond_result, then_block, else_block);

                scopes.function_push_inst(br);

                // 3. set then_block as current block
                scopes.function_add_block(then_block);
                
                // 4. generate body of then branch and seal it
                self.then.generate(scopes);
                
                let jump_end = scopes.new_value().jump(end_block);
                scopes.function_push_inst(jump_end);

                // 5. set else block as current block
                scopes.function_add_block(else_block);

                // 6. generate body of else branch and seal it
                else_then.generate(scopes);

                let jump_end1 = scopes.new_value().jump(end_block);
                scopes.function_push_inst(jump_end1);
            },
            None => {
                // 1. generate branch instruction
                let br = scopes.new_value().branch(cond_result, then_block, end_block);

                scopes.function_push_inst(br);

                // 3. set then_block as current block
                scopes.function_add_block(then_block);
                
                // 4. generate body of then branch and seal it
                self.then.generate(scopes);
                
                let jump_end = scopes.new_value().jump(end_block);
                scopes.function_push_inst(jump_end);
            }
        }
        scopes.function_add_block(end_block);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for While {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        let while_entry = scopes.create_new_block(Some("%While_entry".into()));
        let while_body = scopes.create_new_block(Some("%While_body".into()));
        
        let while_end = scopes.create_new_block(Some("%While_end".into()));
        let jump_entry = scopes.new_value().jump(while_entry);
        scopes.function_push_inst(jump_entry);

        // 1. create new loop context
        scopes.add_loop(while_entry, while_end);

        // 2. set current block as entry block

        scopes.function_add_block(while_entry);

        let cond_result = self
        .condition
        .generate(scopes)?
        .into_int(scopes)?;

        // 3. set branch instruction
        let br = scopes.new_value().branch(cond_result, while_body, while_end);

        scopes.function_push_inst(br);

        
        // 4. set current block as body block
        scopes.function_add_block(while_body);

        // 5. generate statement and seal it
        self.body.generate(scopes)?;

        let jump_entry = scopes.new_value().jump(while_entry);

        scopes.function_push_inst(jump_entry);

        // 6. exit loop
        scopes.minus_loop();

        // 7. set current block as end block

        scopes.function_add_block(while_end);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Break {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        let end_block = scopes.loop_end();
        let jump = scopes.new_value().jump(end_block);
        scopes.function_push_inst(jump);

        let next_block = scopes.create_new_block(None);
        scopes.function_add_block(next_block);
        Ok(())
    }
}

impl<'ast> GenerateIR<'ast> for Continue {
    type Out = ();
    fn generate(&'ast self, scopes : &mut Scopes<'ast>) 
        -> IResult<Self::Out> {
        let start_block = scopes.loop_start();
        let jump = scopes.new_value().jump(start_block);
        scopes.function_push_inst(jump);

        let next_block = scopes.create_new_block(None);
        scopes.function_add_block(next_block);
        Ok(())
    }
}