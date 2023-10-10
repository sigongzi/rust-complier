mod gen;
mod func;
mod context;

use crate::ast::CompUnit;
use gen::GenerateIR;
use koopa::ir::{Program, Type};
use std::fmt;

use self::context::Context;



/// Error returned by IR generator.
pub enum IRError {
    // DuplicatedDef,
    // SymbolNotFound,
    // FailedToEval,
    // InvalidArrayLen,
    // InvalidInit,
    // ArrayAssign,
    // NotInLoop,
    // RetValInVoidFunc,
    // DerefInt,
    // UseVoidValue,
    // ArgMismatch,
    // NonIntCalc,
    NotInFunction
}


impl fmt::Display for IRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self {
        // Self::DuplicatedDef => write!(f, "duplicated symbol definition"),
        // Self::SymbolNotFound => write!(f, "symbol not found"),
        // Self::FailedToEval => write!(f, "failed to evaluate constant"),
        // Self::InvalidArrayLen => write!(f, "invalid array length"),
        // Self::InvalidInit => write!(f, "invalid initializer"),
        // Self::ArrayAssign => write!(f, "assigning to array"),
        // Self::NotInLoop => write!(f, "using break/continue outside of loop"),
        // Self::RetValInVoidFunc => write!(f, "returning value in void fucntion"),
        // Self::DerefInt => write!(f, "dereferencing an integer"),
        // Self::UseVoidValue => write!(f, "using a void value"),
        // Self::ArgMismatch => write!(f, "argument mismatch"),
        // Self::NonIntCalc => write!(f, "non-integer calculation"),
        // }
        Ok(())
    }
}

/// Result type of IR generator.
pub type Result<T> = std::result::Result<T, IRError>;

/// Generates Koopa IR program for the given compile unit (ASTs).
pub fn generate_program(comp_unit: &CompUnit) -> Result<Program> {
    let mut program = Program::new();
    comp_unit.generate(&mut program, &mut Context::new())?;
    Ok(program)
}