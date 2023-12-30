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
    pub func_def: Vec<FuncDef>,
}




/// FuncDef   ::= FuncType IDENT "(" [FuncFParams] ")" Block;


#[derive(Debug)]
pub struct FuncDef {
    pub ty: FuncType,
    pub params : Vec<FuncFParam>,
    pub ident: String,
    pub block: Block,
}


/// FuncFParam ::= BType IDENT;
#[derive(Debug)]
pub struct  FuncFParam {
    pub id : String
}


/// FuncType  ::= "int";
#[derive(Debug)]
pub enum FuncType {
    Int,
    Void
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