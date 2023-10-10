
// AST for CompUnit
// CompUnit  ::= FuncDef;
#[derive(Debug)]
pub struct CompUnit {
    pub func_def: FuncDef,
}


// FuncDef   ::= FuncType IDENT "(" ")" Block;
#[derive(Debug)]
pub struct FuncDef {
    pub func_type: FuncType,
    
    pub ident: String,
    pub block: Block,
}


// FuncType  ::= "int";
#[derive(Debug)]
pub enum FuncType {
    Int
}

// Block     ::= "{" Stmt "}";
#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt
}


// Stmt ::= "return" <Exp> ";";
#[derive(Debug)] 
pub struct Stmt {
    pub exp : Exp
}


// Exp ::= UnaryExp
#[derive(Debug)]
pub struct Exp {
    pub unary : UnaryExp
}


// PrimaryExp  ::= "(" Exp ")" | Number;
#[derive(Debug)]
pub enum PrimaryExp {
    Exp(Box<Exp>),
    Number(Number),
}

// UnaryExp   ::= PrimaryExp | UnaryOp UnaryExp;
#[derive(Debug)]
pub enum UnaryExp {
    Primary(PrimaryExp),
    Unary(UnaryOp, Box<UnaryExp>)
}

// UnaryOp     ::= "+" | "-" | "!";
#[derive(Debug)]
pub enum UnaryOp {
    Negative,
    LNot,
    Positive
}
// Number    ::= INT_CONST;
type Number = i32;