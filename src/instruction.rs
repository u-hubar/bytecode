#[derive(Debug)]
pub enum Instruction {
    LoadValue(isize),
    WriteVariable(String),
    ReadVariable(String),
    Add,
    Sub,
    Multiply,
    Divide,
    ReturnValue,
}

impl From<&[&str]> for Instruction {
    fn from(instr_str: &[&str]) -> Self {
        match instr_str {
            ["LOAD_VAL", val] => Instruction::LoadValue(val.parse::<isize>().unwrap()),
            ["WRITE_VAR", var] => Instruction::WriteVariable(var.replace(&['\'', '"'][..], "")),
            ["READ_VAR", var] => Instruction::ReadVariable(var.replace(&['\'', '"'][..], "")),
            ["ADD"] => Instruction::Add,
            ["SUB"] => Instruction::Sub,
            ["MULTIPLY"] => Instruction::Multiply,
            ["DIVIDE"] => Instruction::Divide,
            ["RETURN_VAL"] => Instruction::ReturnValue,
            invalid_instr => panic!("Invalid instruction: {:?}", invalid_instr),
        }
    }
}
