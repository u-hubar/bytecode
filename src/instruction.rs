#[derive(Debug)]
pub enum Instruction {
    LoadValue(isize),
    WriteVariable(String),
    ReadVariable(String),
    Add,
    Sub,
    Multiply,
    Divide,
    Print,
    PrintVariable(String),
    Label,
    JumpIfEqual(String),
    JumpIfNotEqual(String),
    JumpIfGreater(String),
    JumpIfSmaller(String),
    JumpIfGreaterEqual(String),
    JumpIfSmallerEqual(String),
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
            ["PRINT"] => Instruction::Print,
            ["PRINT", var] => Instruction::PrintVariable(var.replace(&['\'', '"'][..], "")),
            ["LABEL", _] => Instruction::Label,
            ["JUMP_IF_EQ", label_key] => Instruction::JumpIfEqual(label_key.to_string()),
            ["JUMP_IF_NQ", label_key] => Instruction::JumpIfNotEqual(label_key.to_string()),
            ["JUMP_IF_GR", label_key] => Instruction::JumpIfGreater(label_key.to_string()),
            ["JUMP_IF_SM", label_key] => Instruction::JumpIfSmaller(label_key.to_string()),
            ["JUMP_IF_GREQ", label_key] => Instruction::JumpIfGreaterEqual(label_key.to_string()),
            ["JUMP_IF_SMEQ", label_key] => Instruction::JumpIfSmallerEqual(label_key.to_string()),
            ["RETURN_VAL"] => Instruction::ReturnValue,
            invalid_instr => panic!("Invalid instruction: {:?}", invalid_instr),
        }
    }
}
