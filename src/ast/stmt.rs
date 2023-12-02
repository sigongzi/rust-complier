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
    If(Box<If>),
    Break(Break),
    Continue(Continue)
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

/// "break" ";"
#[derive(Debug)]
pub struct Break;

/// "continue" ";"
#[derive(Debug)]
pub struct Continue;

#[derive(Debug)]
pub struct If {
    pub condition: Exp,
    pub then: Stmt,
    pub else_then: Option<Stmt>
}