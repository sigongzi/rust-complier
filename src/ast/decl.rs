use super::{Exp, ConstExp};
// Decl          ::= ConstDecl;
#[derive(Debug)]
pub enum Decl {
    Const(ConstDecl),
    Var(VarDecl)
}

// BType         ::= "int";
// VarDecl       ::= BType VarDef {"," VarDef} ";";
#[derive(Debug)]
pub struct VarDecl {
    pub defs: Vec<VarDef>
}

// VarDef        ::= IDENT | IDENT "=" InitVal;
#[derive(Debug)]
pub struct VarDef {
    pub id : String,
    pub init_val : Option<InitVal>
}

// InitVal       ::= Exp;
#[derive(Debug)]
pub struct InitVal {
    pub exp : Exp
}

// ConstDecl     ::= "const" BType ConstDef {"," ConstDef} ";";
#[derive(Debug)]
pub struct ConstDecl {
    pub defs: Vec<ConstDef>
}

// ConstDef      ::= IDENT "=" ConstInitVal;
#[derive(Debug)]
pub struct ConstDef {
    pub id : String,
    pub init_val : ConstInitVal
}

// ConstInitVal  ::= ConstExp;
#[derive(Debug)]
pub struct ConstInitVal {
    pub exp : ConstExp,
}