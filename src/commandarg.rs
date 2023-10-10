use crate::MainError;
use std::str::FromStr;
use std::env::args;
// Command line arguments.
pub struct CommandLineArgs {
    pub mode: Mode,
    pub input: String,
    pub output: String,
}

// Compile mode.
pub enum Mode {
    // Compile SysY to Koopa IR.
    Koopa,
    // Compile SysY to RISC-V assembly.
    Riscv,
    // Compile SysY to optimized RISC-V assembly.
    Perf,
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-koopa" => Ok(Mode::Koopa),
            "-riscv" => Ok(Mode::Riscv),
            "-perf" => Ok(Mode::Perf),
            _ => Err(()),
        }
    }
}

impl CommandLineArgs {
    pub fn parse() -> Result<Self, MainError> {
        let mut args = args();
        args.next();
        let mode = args.next().unwrap().parse::<Mode>().unwrap();
        let input = args.next().unwrap();
        match args.next() {
            Some(c) => {
                if c != "-o" {
                    return Err(MainError::InvalidArgs);
                }
            }
            None => {return Err(MainError::InvalidArgs);}
        };
        let output = args.next().unwrap();
        Ok(
            Self {
                mode,
                input,
                output,
            }
        )
    }
}