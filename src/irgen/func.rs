use koopa::ir::Value;
use koopa::ir::builder::LocalBuilder;
use koopa::ir::{BasicBlock, Function, Program};


pub struct FunctionHandler {
    func: Function,
    entry: BasicBlock
}

impl FunctionHandler {
    pub fn new(func : Function, entry : BasicBlock) -> Self {
        Self {
            func,
            entry
        }
    }


    // Creates a new value in function.
    pub fn new_value<'p>(&self, program: &'p mut Program) -> LocalBuilder<'p> {
        program.func_mut(self.func).dfg_mut().new_value()
    }

    // push a instance in a block
    pub fn push_inst_to(&self, program: &mut Program, bb: BasicBlock, inst: Value) {
        program
        .func_mut(self.func)
        .layout_mut()
        .bb_mut(bb)
        .insts_mut()
        .push_key_back(inst)
        .unwrap();
    }

    // push a instance 
    pub fn push_inst_to_entry(&self, program: &mut Program, inst: Value) {
        self.push_inst_to(program, self.entry, inst)

    }
}