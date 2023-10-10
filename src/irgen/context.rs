
use super::func::FunctionHandler;

pub struct Context {
    pub current_func: Option<FunctionHandler>
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