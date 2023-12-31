mod gen;
//mod _func;
mod func;
mod scopes;
mod constcalc;
use crate::ast::CompUnit;
use gen::GenerateIR;
use koopa::ir::Program;
use scopes::Scopes;
use std::fmt;




/// Error returned by IR generator.
pub enum IRError {

    NotMemory,
    AdvancedEvaluation(String),
    VoidValue,
    UndefinedLVal(String),
    EvaluateConstWithCall,
    EvaluateConstWithVar,
}


impl fmt::Display for IRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VoidValue => write!(f,"use void value in an expression"),
            Self::AdvancedEvaluation(s) => write!(f, "wrong when evaluate binary operator: {}", s),
            Self::UndefinedLVal(s) => write!(f, "{} is undefined", s),
            Self::NotMemory => write!(f, "store val in a place not memory"),
            Self::EvaluateConstWithCall => write!(f, "can not use function call in const variable defination"),
            Self::EvaluateConstWithVar => write!(f, "can not use variable in const variable defination")
        }
    }
}

/// Result type of IR generator.
pub type IResult<T> = std::result::Result<T, IRError>;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> IResult<Program> {
    let mut program = Program::new();
    comp_unit.generate(&mut Scopes::new(&mut program))?;
    Ok(program)
}