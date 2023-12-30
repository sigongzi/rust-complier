use koopa::ir::{Program, BasicBlock, Value, entities::ValueData, Function, Type};
use super::{func::{FunctionHandler,FuncVar}, CResult};

#[macro_export]
macro_rules! function_handler {
    ($context: ident) => {
        $context.cur_func.as_mut().unwrap()
    };
}
pub struct ProgramContext<'p> {
    program: &'p Program,
    pub cur_func : Option<FunctionHandler>,
    pub func : Option<Function>,
    function_cnt : usize,
}

#[allow(unused)]
impl<'p> ProgramContext<'p> {
    pub fn new(program: &'p Program) -> Self{
        Self { 
            program, 
            cur_func: None,
            func : None,
            function_cnt : 0
        }
    }
    pub fn program(&'p self) -> &'p Program {
        self.program
    }

    /// get function handler
    pub fn get_func_handler(&'p mut self) -> Option<&mut FunctionHandler> {
        self.cur_func.as_mut()
    }

    /// get value_data
    pub fn get_value_data(&self, value : &Value) -> &ValueData{
        self.program
        .func(self.func.unwrap())
        .dfg()
        .value(*value)
    }

    // get function name in dfg by Function
    pub fn get_function_name(&self, func : Function) -> &str {
        self
        .program
        .func(func)
        .name()
    }
    // get value position in current function
    pub fn get_value_position(&self, value : &Value) -> CResult<FuncVar> {
        self
        .cur_func
        .as_ref()
        .unwrap()
        .get_value_position(value)
    }

    pub fn get_function_cnt(&self) -> usize {
        self.function_cnt
    }


    pub fn set_current_function(&mut self, f : FunctionHandler) {
        self.function_cnt += 1;
        self.cur_func = Some(f);
    }

    // set value allocation for the current function
    pub fn set_function_value_pos(&mut self, value : Value, pos : usize) {
        self
        .cur_func
        .as_mut()
        .unwrap()
        .set_value_pos(value, pos);
    }
    pub fn get_basic_block_name(&mut self, bb: BasicBlock) -> &str {
        self.cur_func.as_mut().unwrap().get_basic_block_name(bb)
    }
}
