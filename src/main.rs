use std::{env::args, fs::File, io::Read};

use bytecode::{parser::Parser, vm::VirtualMachine};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();
    let mut f = File::open(&args[1])?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let bytecode = Parser::parse_code(&buffer);
    let labels = Parser::parse_labels(&bytecode);
    let variables = Parser::parse_variables(&bytecode);
    let program = Parser::parse_instructions(&bytecode, &labels, &variables);

    let mut vm = VirtualMachine::new();
    vm.run(program);

    Ok(())
}
