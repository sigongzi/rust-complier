
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

/// Block     ::= "{" Stmt "}";
#[derive(Debug)]
pub struct Block {
    pub stmt: Stmt
}


/// Stmt ::= "return" <Exp> ";";
#[derive(Debug)] 
pub struct Stmt {
    pub exp : Exp
}

/// Number    ::= INT_CONST;
type Number = i32;

/// Expression
/// Exp ::= LOrExp
#[derive(Debug)]
pub struct Exp {
    pub lor : LOrExp
}


/// PrimaryExp  ::= "(" Exp ")" | Number;
#[derive(Debug)]
pub enum PrimaryExp {
    Ausdruck(Box<Exp>),
    Number(Number),
}

/// UnaryExp   ::= PrimaryExp | UnaryOp UnaryExp;
#[derive(Debug)]
pub enum UnaryExp {
    PrimaryAusdruck(PrimaryExp),
    UnaryAusdruck(UnaryOp, Box<UnaryExp>)
}

/// MulExp      ::= UnaryExp | MulExp MulOp UnaryExp;
#[derive(Debug)]
pub enum MulExp {
    UnaryAusdruck(UnaryExp),
    MulAusdruck(Box<MulExp>, MulOp, UnaryExp)
}

/// AddExp ::= MulExp | AddExp AddOp MulExp
#[derive(Debug)]
pub enum AddExp {
    MulAusdruck(MulExp),
    AddAusdruck(Box<AddExp>, AddOp, MulExp)
}

/// RelExp ::=  AddExp | RelExp ("<" | ">" | "<=" | ">=") AddExp
#[derive(Debug)]
pub enum RelExp {
    AddAusdruck(AddExp),
    RelAusdruck(Box<RelExp>, RelOp, AddExp)
}

/// EqExp ::= RelExp | EqExp ("==" | "!=") RelExp;
#[derive(Debug)]
pub enum EqExp {
    RelAusdruck(RelExp),
    EqAusdruck(Box<EqExp>, EqOp, RelExp)
}

/// EqExp | LAndExp "&&" EqExp;
#[derive(Debug)]
pub enum LAndExp {
    EqAusdruck(EqExp),
    LAndAusdruck(Box<LAndExp>, EqExp)
}

/// LOrExp      ::= LAndExp | LOrExp "||" LAndExp;
#[derive(Debug)]
pub enum LOrExp {
    LAndAusdruck(LAndExp),
    LOrAusdruck(Box<LOrExp>, LAndExp)
}

/// Operator: From high level to low
/// UnaryOp     ::= "+" | "-" | "!";
#[derive(Debug)]
pub enum UnaryOp {
    Negative,
    LNot,
    Positive
}

/// MulOp ::= "*" | "/" | "%"
#[derive(Debug)]
pub enum MulOp {
    Mul,
    Div,
    Mod
}

/// AddOp ::= "+" | "-"
#[derive(Debug)]
pub enum AddOp {
    Add,
    Sub
}

/// RelOp ::= "<" | ">" | "<=" | ">="
#[derive(Debug)]
pub enum RelOp {
    Lt,
    Gt,
    Le,
    Ge
}

/// EqOp ::= "==" | "!="
#[derive(Debug)]
pub enum EqOp {
    Eq,
    NotEq
}