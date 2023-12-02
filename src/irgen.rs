mod gen;
//mod _func;
mod func;
mod scopes;
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
}


impl fmt::Display for IRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::VoidValue => write!(f,"use void value in an expression"),
            Self::AdvancedEvaluation(s) => write!(f, "wrong when evaluate binary operator: {}", s),
            Self::UndefinedLVal(s) => write!(f, "{} is undefined", s),
            Self::NotMemory => write!(f, "store val in a place not memory")
        }
    }
}

/// Result type of IR generator.
pub type Result<T> = std::result::Result<T, IRError>;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    comp_unit.generate(&mut Scopes::new(&mut program))?;
    Ok(program)
}