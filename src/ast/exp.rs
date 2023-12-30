use super::{LVal, Number};
/// Expression
/// ConstExp      ::= Exp;
#[derive(Debug)]
pub struct ConstExp {
    pub exp: Exp,
}

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
    LVal(LVal)
}

/// UnaryExp   ::= PrimaryExp | UnaryOp UnaryExp | IDENT "(" [FuncRParams] ")"
#[derive(Debug)]
pub enum UnaryExp {
    PrimaryAusdruck(PrimaryExp),
    UnaryAusdruck(UnaryOp, Box<UnaryExp>),
    Call(FuncCall)
}
#[derive(Debug)]
pub struct FuncCall {
    pub id : String,
    pub params: Vec<Exp>
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