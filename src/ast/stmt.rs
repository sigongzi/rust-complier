use super::{LVal,Exp, Block};
///Stmt          ::= LVal "=" Exp ";"
///                | "return" [Exp] ";"
///                | Block
///                | [Exp] ";"
#[derive(Debug)] 
pub enum Stmt {
    Return(Return),
    Assign(Assign),
    ExpStmt(ExpStmt),
    Block(Block),
}

// [Exp] ";"
#[derive(Debug)]
pub struct ExpStmt {
    pub exp: Option<Exp>
}

/// "return" Exp ";"
#[derive(Debug)]
pub struct Return {
    pub exp : Option<Exp>
}

/// LVal "=" Exp ";"
#[derive(Debug)]
pub struct Assign {
    pub lval : LVal,
    pub exp : Exp
}