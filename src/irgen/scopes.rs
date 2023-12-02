use koopa::ir::builder::LocalBuilder;
use koopa::ir::builder_traits::{LocalInstBuilder, BasicBlockBuilder};
use koopa::ir::{Value, Function, BasicBlock, Program, Type};
use std::collections::HashMap;
use std::vec::Vec;
use super::func::FunctionInfo;
use super::{Result, IRError};

// Value: ValueId
// Function: FunctionId
// both need program to find their entities
pub struct Scopes<'ast> {
    val_level: Vec<HashMap<&'ast str, Value>>,
    function_map: HashMap<&'ast str, Function>,
    pub program: &'ast mut Program,
    pub cur_func: Option<FunctionInfo>,
    pub loop_level: Vec<(BasicBlock, BasicBlock)>
} 

impl<'ast> Scopes<'ast> {
    pub fn new(program: &'ast mut Program) -> Scopes<'ast> {
        Self {
            val_level: vec![],
            function_map: HashMap::new(),
            program,
            cur_func: None,
            loop_level: vec![]
        }
    }
    pub fn record_function(&mut self, ident: &'ast str, func_id: Function) {
        self.function_map.insert(ident, func_id);
    }

    pub fn add_level(&mut self) {
        self.val_level.push(HashMap::new());
    }
    pub fn minus_level(&mut self) {
        self.val_level.pop().unwrap();
    }

    pub fn add_loop(&mut self, entry: BasicBlock, end: BasicBlock) {
        self.loop_level.push((entry, end));
    }
    pub fn minus_loop(&mut self) {
        self.loop_level.pop();
    }

    pub fn loop_start(&self) -> BasicBlock{
        self.loop_level
        .last()
        .unwrap()
        .0
    }
    pub fn loop_end(&self) -> BasicBlock{
        self.loop_level
        .last()
        .unwrap()
        .1
    }

    pub fn get_func_id(&self) -> Function {
        self.cur_func.as_ref().unwrap().func
    }

    pub fn set_func_cur(&mut self, block_id: BasicBlock) {
        self.cur_func.as_mut().unwrap().cur = block_id
    }

    pub fn get_current_block(&self) -> BasicBlock {
        self.cur_func.as_ref().unwrap().cur
    }

    pub fn get_entry(&self) -> BasicBlock {
        self.cur_func.as_ref().unwrap().entry
    }

    pub fn get_end(&self) -> BasicBlock {
        self.cur_func.as_ref().unwrap().end
    }

    pub fn get_ret(&self) -> Option<Value> {
        self.cur_func.as_ref().unwrap().ret
    }

    pub fn retrieve_val(&self, s: &str) -> Option<Value>{
        for h in self.val_level.iter().rev() {
            if let Some(v) = h.get(s) {
                return Some(v.clone());
            }
        }
        None
    }

    pub fn new_value(&mut self) -> LocalBuilder {
        self.program
        .func_mut(self.get_func_id())
        .dfg_mut()
        .new_value()
    }

    // close entry of a function
    pub fn close_entry(&mut self, next: BasicBlock) {
        let jump = self.new_value().jump(next);
        let entry = self.get_entry();
        self.function_push_inst_to_block(jump, entry);
    }

    // close end block of a function
    pub fn close_function(&mut self, end: BasicBlock) {
        let end = self.get_end();
        let jump = self.new_value().jump(end);
        self.function_push_inst(jump);
        
        self.function_add_block(end);
        let res = match self.get_ret() {
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



    // function operation
    pub fn function_add_block(&mut self, block_id: BasicBlock) {
        self.program
        .func_mut(self.get_func_id())
        .layout_mut()
        .bbs_mut()
        .push_key_back(block_id)
        .unwrap();
        self.set_func_cur(block_id);
    }

    fn function_push_inst_to_block(&mut self, inst: Value, block_id: BasicBlock) {
        self.program
        .func_mut(self.get_func_id())
        .layout_mut()
        .bb_mut(block_id)
        .insts_mut()
        .push_key_back(inst)
        .unwrap();
    }
    pub fn function_push_inst(&mut self, inst: Value) {
        let cur = self.get_current_block();
        self.function_push_inst_to_block(inst, cur);
    }

    pub fn create_initial_variable(&mut self, ty: Type, name: Option<&str>) -> Value{

        let alloc = self.new_value().alloc(ty);
        if let Some(name) = name {
            self.program
            .func_mut(self.get_func_id())
            .dfg_mut()
            .set_value_name(alloc, Some(format!("@{}", name)));
        }
        self.function_push_inst(alloc);
        alloc
    }

    // create a new block (but doesn't add it)
    pub fn create_new_block(&mut self, name : Option<String>) -> BasicBlock {
        self.program
        .func_mut(self.get_func_id())
        .dfg_mut()
        .new_bb()
        .basic_block(name)
    }

    // add const val into hash set
    pub fn add_variable_name(&mut self, name: &'ast str, val: Value) {
        self.val_level
        .last_mut()
        .unwrap()
        .insert(name, val);
    }
}
