use koopa::ir::{BasicBlock, Value};
use std::collections::HashMap;

use super::{CResult, GenerateError};

pub struct FunctionHandler {
    stack_var : HashMap<Value, usize>,
    parameters : HashMap<Value, usize>,
    name : HashMap<BasicBlock, String>,
    func_id : usize,
    name_counter : i32,
    stack_size : usize,
}

pub enum FuncVar {
    Reg(String),
    Stack(usize)
}

impl FunctionHandler {
    pub fn new(func_id : usize, params : Vec<Value>, stack_size : usize) -> Self {
        Self {
            stack_var : HashMap::new(),
            parameters : params
                .iter()
                .enumerate()
                .map(|(i,e)| (e.to_owned(),i))
                .collect(),
            name : HashMap::new(),
            name_counter : 0,
            func_id,
            stack_size
        }
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


    // get value position in stack or as a parameter
    pub fn get_value_position(&self, value: &Value) -> CResult<FuncVar>{
        match self.parameters.get(value).copied() {
            Some(pos) => {
                match pos {
                    v if v < 8 => {
                        Ok(FuncVar::Reg(format!("a{}",v)))   
                    },
                    v => {
                        Ok(FuncVar::Stack(self.stack_size + (v - 8) * 4))
                    }
                }
            },
            None => {
                match self.stack_var.get(value).copied() {
                    Some(pos) => Ok(FuncVar::Stack(pos)),
                    None => Err(GenerateError::NoValue)
                }
            }
        }
    }
}