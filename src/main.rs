use std::{env::args, fs::File, io::Read};

use bytecode::{parser::Parser, vm::VM};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let mut f = File::open(&args[1])?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let lines = buf
        .split("\n")
        .map(|line| line.trim().split(" ").filter(|token| !token.is_empty()).collect::<Vec<_>>())
        .filter(|line_vec| !line_vec.is_empty())
        .collect::<Vec<_>>();

    let instructions = Parser::parse_code(lines);

    let mut vm = VM::new();
    vm.run(instructions);

    Ok(())
}
