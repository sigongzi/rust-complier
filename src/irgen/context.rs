
use super::func::FunctionHandler;

pub struct Context {
    pub current_func: Option<FunctionHandler>
}

#[macro_export]
macro_rules! cur_func {
    ($context: ident) => {
        $context.current_func.as_mut().unwrap()
    };
}

impl Context {
    pub fn new() -> Self {
        Self { 
            current_func: None 
        }
    }
    pub fn get_current_func(&mut self) -> &mut FunctionHandler{
        self.current_func.as_mut().unwrap()
    }
}