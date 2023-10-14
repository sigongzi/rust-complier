use super::{LVal,Exp};
///Stmt          ::= LVal "=" Exp ";"
///                | "return" Exp ";";
#[derive(Debug)] 
pub enum Stmt {
    Return(Return),
    Assign(Assign)
}

/// "return" Exp ";"
#[derive(Debug)]
pub struct Return {
    pub exp : Exp
}

/// LVal "=" Exp ";"
#[derive(Debug)]
pub struct Assign {
    pub lval : LVal,
    pub exp : Exp
}