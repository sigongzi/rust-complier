mod gen;
mod context;
mod asmwriter;
mod func;
use koopa::ir::Program;
use std::fmt;
use std::fs::File;

use self::context::ProgramContext;
use self::gen::GenerateAsm;

pub type Result<T> = std::result::Result<T, GenerateError>;
pub fn generate_asm(program : &Program, path : &str) -> Result<()> {
    
    program.generate_asm(&mut File::create(path)?, &mut ProgramContext::new(program))?;
    Ok(())
}



pub enum GenerateError {
    IOError(std::io::Error),
    Unimplemented,
    MissingCurrentFunction(String),
    NoValue
}


impl From<std::io::Error> for GenerateError {
    fn from(error: std::io::Error) -> Self {
        GenerateError::IOError(error)
    }
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(e) => write!(f, "wrong in writing assembly to file: {}", e),
            Self::Unimplemented => write!(f,"other conditions are unimplemented"),
            Self::MissingCurrentFunction(e) => write!(f,"command {} can't see the function", e),
            Self::NoValue => write!(f,"Can't find the target value in stack")
        }
    }
}
