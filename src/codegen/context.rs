use koopa::ir::{Program, BasicBlock, Value, entities::ValueData};
use super::func::FunctionHandler;

#[macro_export]
macro_rules! function_handler {
    ($context: ident) => {
        $context.cur_func.as_mut().unwrap()
    };
}
pub struct ProgramContext<'p> {
    program: &'p Program,
    pub cur_func : Option<FunctionHandler>,
    function_cnt : usize,
}

#[allow(unused)]
impl<'p> ProgramContext<'p> {
    pub fn new(program: &'p Program) -> Self{
        Self { 
            program, 
            cur_func: None,
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
        .func(self.cur_func.as_ref().unwrap().get_func())
        .dfg()
        .value(*value)
    }


    pub fn get_function_cnt(&self) -> usize {
        self.function_cnt
    }
    pub fn set_current_function(&mut self, f : FunctionHandler) {
        self.function_cnt += 1;
        self.cur_func = Some(f);
    }

    pub fn get_basic_block_name(&mut self, bb: BasicBlock) -> &str {
        self.cur_func.as_mut().unwrap().get_basic_block_name(bb)
    }
}
