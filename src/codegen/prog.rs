use koopa::ir::{Function, Program};

pub struct ProgramHandler<'p> {
    program: &'p Program,
    cur_func : Option<Function>
}

impl<'p> ProgramHandler<'p> {
    pub fn new(program: &'p Program) -> Self{
        Self { 
            program, 
            cur_func: None 
        }
    }
    pub fn program(&'p self) -> &'p Program {
        self.program
    }
    pub fn cur_func(&self) -> Option<Function> {
        self.cur_func
    }

    pub fn set_current_function(&mut self, f : Function) {
        self.cur_func = Some(f);
    }
}
