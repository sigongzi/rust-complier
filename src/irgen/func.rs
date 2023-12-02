use koopa::ir::{BasicBlock, Function, Value};

pub struct FunctionInfo {
    pub func: Function,
    pub entry: BasicBlock,
    pub cur: BasicBlock,
    pub end: BasicBlock,
    pub ret: Option<Value>,
}

impl FunctionInfo {
    pub fn new(func : Function, entry : BasicBlock, cur: BasicBlock, end : BasicBlock, ret: Option<Value>) -> Self {
        Self {
            func,
            entry,
            cur,
            end,
            ret,
        }
    }

    pub fn get_id(&self) -> Function {
        self.func
    }

    pub fn set_cur(&mut self, block_id : BasicBlock) {
        self.cur = block_id;
    }

    pub fn get_current_block(&self) -> BasicBlock {
        self.cur
    }
}
