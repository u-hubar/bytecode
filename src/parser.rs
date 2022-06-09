use crate::instruction::Instruction;

pub struct Parser;

impl Parser {
    pub fn parse_code(code: Vec<Vec<&str>>) -> Vec<Instruction> {
        code.iter()
            .map(|line| Instruction::from(line.as_slice()))
            .collect::<Vec<_>>()
    }
}
