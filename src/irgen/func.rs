use koopa::ir::{Value, TypeKind};
use koopa::ir::builder::{LocalBuilder,LocalInstBuilder};
use koopa::ir::builder_traits::BasicBlockBuilder;
use koopa::ir::entities::{ValueData};
use koopa::ir::{BasicBlock, Function, Program, Type};
use std::collections::HashMap;



pub struct FunctionHandler {
    func: Function,
    entry: BasicBlock,
    cur: BasicBlock,
    end: BasicBlock,
    ret: Option<Value>,
    const_val : HashMap<String, Value>,
}

impl FunctionHandler {
    pub fn new(func : Function, entry : BasicBlock, cur: BasicBlock, end : BasicBlock) -> Self {
        Self {
            func,
            entry,
            cur,
            end,
            ret : None,
            const_val : HashMap::new()
        }
    }


    // Creates a new value in function.
    pub fn new_value<'p>(&self, program: &'p mut Program) -> LocalBuilder<'p> {
        program.func_mut(self.func).dfg_mut().new_value()
    }

    // push a instance in a block
    pub fn push_inst_to(&self, program: &mut Program, block: BasicBlock, inst: Value) {
        program
        .func_mut(self.func)
        .layout_mut()
        .bb_mut(block)
        .insts_mut()
        .push_key_back(inst)
        .unwrap();
    }

    // push instruction in current block
    pub fn push_inst<'p>(&self, program: &'p mut Program, inst: Value) {
        self.push_inst_to(program, self.cur, inst)
    }

    // create a new block
    pub fn create_new_block<'p>(&self, program: &'p mut Program, name : Option<String>) -> BasicBlock {
        program
        .func_mut(self.func)
        .dfg_mut()
        .new_bb()
        .basic_block(name)
    }

    // add a new block
    pub fn add_new_block<'p>(&mut self, program: &'p mut Program, new_block : BasicBlock) {
        program
        .func_mut(self.func)
        .layout_mut()
        .bbs_mut()
        .push_key_back(new_block)
        .unwrap();
        self.cur = new_block;
    }

    // create a initial variable in this function
    pub fn create_initial_variable<'p>(&self, program : &'p mut Program, ty: Type, name: Option<&str>) -> Value{
        let alloc = self.new_value(program).alloc(ty);
        if let Some(name) = name {
            program
            .func_mut(self.func)
            .dfg_mut()
            .set_value_name(alloc, Some(format!("@{}", name)));
        }
        self.push_inst_to(program, self.entry, alloc);
        alloc
    }

    // get value data
    pub fn get_value_data<'p>(&'p self, program : &'p Program, value : Value) -> &ValueData {
        program
        .func(self.func)
        .dfg()
        .value(value)
    }

    // get end block id
    pub fn get_end(&self) -> BasicBlock {
        self.end
    }

    // set return val
    pub fn set_ret_val(&mut self, value : Value) {
        self.ret = Some(value);
    } 

    // get return value
    pub fn get_ret_value(&mut self) -> Value{
        self.ret.unwrap()
    }

    // get funtion kind
    pub fn get_function_kind<'a>(&'a self, program : &'a Program) -> &TypeKind {
        match 
        program.
        func(self.func)
        .ty()
        .kind() {
            TypeKind::Function(_f, t) => t.kind(),
            _ => unreachable!()
        }
    }

    // add const val into hash set
    pub fn add_const_val(&mut self, name: String, val: Value) {
        self.const_val.insert(name, val);
    }

    // require if the val is in const hashset 
    pub fn require_val(&mut self, name: &str) -> Option<Value>{
        self.const_val.get(name).copied()
    }
}