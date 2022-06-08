use crate::instruction::Instruction;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: Vec<Vec<&str>>) {
        for line in code {
            match Instruction::from(line.as_slice()) {
                Instruction::LoadValue(val) => println!("Load value {}", val),
                Instruction::ReturnValue => println!("Return value"),
                Instruction::WriteVariable(var) => println!("Write variable {}", var),
                Instruction::ReadVariable(var) => println!("Read variable {}", var),
                Instruction::Add => println!("Add"),
                Instruction::Sub => println!("Sub"),
                Instruction::Multiply => println!("Multiply"),
                Instruction::Divide => println!("Divide"),
            }
        }
    }
}
