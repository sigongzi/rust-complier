use koopa::ir::{BasicBlock, Function, Value};
use std::collections::HashMap;

use super::{Result, GenerateError};

pub struct FunctionHandler {
    func : Function,
    stack_var : HashMap<Value, usize>,
    name : HashMap<BasicBlock, String>,
    func_id : usize,
    name_counter : i32,
}

impl FunctionHandler {
    pub fn new(func : Function, func_id : usize) -> Self {
        Self {
            func,
            stack_var : HashMap::new(),
            name : HashMap::new(),
            name_counter : 0,
            func_id
        }
    }

    pub fn get_func(&self) -> Function {
        return self.func;
    }

    // get basic block data
    pub fn get_basic_block_name(&mut self, bb: BasicBlock) -> &str {
        if !self.name.contains_key(&bb) {
            self.name_counter += 1;
            let new_name = format!(".L{}_{}", self.func_id, self.name_counter);
            self.name.insert(bb, new_name);
        }
        &self.name.get(&bb).unwrap()
    }

    //pub set_value_pos
    pub fn set_value_pos(&mut self, value: Value, pos : usize) {
        self.stack_var.insert(value, pos);
    }


    // get value position in stack
    pub fn get_value_pos(&self, value: &Value) -> Result<usize>{
        match self.stack_var.get(value) {
            Some(pos) => Ok(*pos),
            None => Err(GenerateError::NoValue),
        }
    }
}