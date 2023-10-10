// cargo run -- -koopa hello.c -o hello.koopa

/// usage: compiler -[koopa/riscv/perf] <INPUT_FILE> -o <OUTPUT_FILE>
/// test: autotest [-h] [-koopa | -riscv | -perf] [-t TEST_CASE_DIR] [-w WORKING_DIR] [-s SUB_DIR] repo_dir
/// test lv3 : autotest -koopa -s lv3 ./
/// autotest -riscv -s lv3 ./
mod ast;
mod irgen;
mod codegen;
mod commandarg;

use koopa::back::KoopaGenerator;
use koopa::ir::Program;
use lalrpop_util::lalrpop_mod;
use std::fs::read_to_string;
use std::process::exit;
use std::{fmt, io};
use commandarg::Mode;

use crate::commandarg::CommandLineArgs;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy
lalrpop_mod!(sysy);


/// Error returned by `main` procedure.
pub enum MainError {
    InvalidArgs,
    File(io::Error),
    // may add parse error here
    Parse,
    IR(irgen::IRError),
    CodeGen(codegen::GenerateError),
    Io(io::Error),
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        exit(-1);
    }
}

fn get_text_ir(program : &Program) {
    let mut gen = KoopaGenerator::new(Vec::new());
    gen.generate_on(&program).unwrap();
    let text_form_ir = std::str::from_utf8(&gen.writer()).unwrap().to_string();
    println!("{}", text_form_ir);
}

fn try_main() -> Result<(), MainError> {
    // 解析命令行参数
    let CommandLineArgs {
        mode,
        input : input_file,
        output : output_file
    } = CommandLineArgs::parse()?;

    // 读取输入文件
    let input = read_to_string(input_file).map_err(MainError::File)?;

    // 1.parser part 
    // 调用 lalrpop 生成的 parser 解析输入文件
    let comp_unit = sysy::CompUnitParser::new()
    .parse(&input).map_err(|_| MainError::Parse)?;

    // 输出解析得到的 AST
    println!("{:#?}", comp_unit);
    
    // 2. ir generator
    let program = irgen::generate_program(&comp_unit).map_err(MainError::IR)?;


    // 输出生成的ir（用来调试）
    get_text_ir(&program);

    // If mode is koopa, generate koopa in the target file
    if matches!(mode, Mode::Koopa) {
        return KoopaGenerator::from_path(&output_file)
          .map_err(MainError::File)?
          .generate_on(&program)
          .map_err(MainError::Io);
    }
    
    // 3. generate RISC-V assembly
    codegen::generate_asm(&program, &output_file).map_err(MainError::CodeGen)?;
    Ok(())
}

impl fmt::Display for MainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
        Self::InvalidArgs => write!(f,"invalid args, please use\n
compiler <MODE>[-koopa,-riscv,-perf] <INPUT> -o <OUTPUT>\n"),
        Self::File(err) => write!(f, "invalid input SysY file: {}", err),
        Self::Parse => write!(f, "error occurred while parsing"),
        Self::IR(err) => write!(f, "{}", err),
        Self::CodeGen(err) => write!(f,"{}", err),
        Self::Io(err) => write!(f, "I/O error: {}", err),
        }
    }
}



