mod stmt;
mod exp;
mod decl;
pub use stmt::{*};
pub use exp::{*};
pub use decl::{*};
// AST for CompUnit
/// CompUnit  ::= FuncDef;
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}




/// FuncDef   ::= FuncType IDENT "(" ")" Block;
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    
    pub ident: String,
    pub block: Block,
}


/// FuncType  ::= "int";
#[derive(Debug)]
pub enum FuncType {
    Int
}

//Block         ::= "{" {BlockItem*} "}";
#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>
}

/// BlockItem     ::= Decl | Stmt;
#[derive(Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt)
}



/// Number    ::= INT_CONST;
type Number = i32;

///LVal
/// LVal          ::= IDENT;
#[derive(Debug)]
pub struct LVal {
    pub id: String
}