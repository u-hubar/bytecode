#[derive(Debug)]
pub enum Instruction {
    LoadValue(isize),
    ReturnValue,
    WriteVariable(String),
    ReadVariable(String),
    Add,
    Sub,
    Multiply,
    Divide,
}

impl From<&[&str]> for Instruction {
    fn from(instr_str: &[&str]) -> Self {
        match instr_str {
            ["LOAD_VAL", val] => Instruction::LoadValue(val.parse::<isize>().unwrap()),
            ["RETURN_VAL"] => Instruction::ReturnValue,
            ["WRITE_VAR", var] => Instruction::WriteVariable(var.replace(&['\'', '"'][..], "")),
            ["READ_VAR", var] => Instruction::ReadVariable(var.replace(&['\'', '"'][..], "")),
            ["ADD"] => Instruction::Add,
            ["SUB"] => Instruction::Sub,
            ["MULTIPLY"] => Instruction::Multiply,
            ["DIVIDE"] => Instruction::Divide,
            invalid_instr => panic!("Invalid instruction: {:?}", invalid_instr),
        }
    }
}
