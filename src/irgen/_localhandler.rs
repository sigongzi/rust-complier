use koopa::ir::{Program, Value,BasicBlock, Function, Type};
use koopa::ir::builder::{LocalBuilder,LocalInstBuilder};
use koopa::ir::builder_traits::BasicBlockBuilder;
use super::scopes::{self, Scopes};
use koopa::ir::BasicBlock;
pub struct LocalHandler<'a,'b> {
    program: &'b mut Program,
    scopes: &'b mut Scopes<'a>
}

impl<'a,'b> LocalHandler<'a,'b> {
    pub fn new(program: &'b mut Program, scopes: &'b mut Scopes<'a>) -> LocalHandler<'a,'b> {
        Self {
            program,
            scopes
        }
    }
    pub fn function_add_block(&mut self, block_id: BasicBlock) {
        self.program
        .func_mut(self.scopes.get_func_id())
        .layout_mut()
        .bbs_mut()
        .push_key_back(block_id)
        .unwrap();
        self.scopes.set_func_cur(block_id);
    }

    fn function_push_inst_to_block(&mut self, inst: Value, block_id: BasicBlock) {
        self.program
        .func_mut(self.scopes.get_func_id())
        .layout_mut()
        .bb_mut(block_id)
        .insts_mut()
        .push_key_back(inst)
        .unwrap();
    }
    pub fn function_push_inst(&mut self, inst: Value) {
        let cur = self.scopes.get_current_block();
        self.function_push_inst_to_block(inst, cur);
    }

    // Creates a new value in function.
    pub fn new_value(&mut self) -> LocalBuilder {
        self.program
        .func_mut(self.scopes.get_func_id())
        .dfg_mut()
        .new_value()
    }

    pub fn close_entry(&mut self, next: BasicBlock) {
        let jump = self.new_value().jump(next);
        let entry = self.scopes.get_entry();
        self.function_push_inst_to_block(jump, entry);
    }

    pub fn close_function(&mut self, end: BasicBlock) {
        let end = self.scopes.get_end();
        let jump = self.new_value().jump(end);
        self.function_push_inst(jump);
        self.function_add_block(end);
        let res = match self.scopes.get_ret() {
            Some(v) => {
                let load = self.new_value().load(v);
                self.function_push_inst(load);
                Some(load)
            },
            None => None
        };

        let ret = self.new_value().ret(res);
        self.function_push_inst(ret);
    }

    pub fn create_initial_variable(&self, ty: Type, name: Option<&str>) {
        let alloc = self.new_value().alloc(ty);
        if let Some(name) = name {
            program
            .func_mut(self.scopes.get_func_id())
            .dfg_mut()
            .set_value_name(alloc, Some(format!("@{}", name)));
        }
        self.push_inst_to(program, self.scopes.get_entry(), alloc);
        alloc
    }
}