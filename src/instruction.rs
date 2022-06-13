use crate::{labels::Labels, variables::Variables};

#[derive(Debug)]
pub enum Instruction {
    LoadValue(isize),
    WriteVariable(usize),
    ReadVariable(usize),
    Add,
    Sub,
    Multiply,
    Divide,
    Print,
    PrintVariable(String, usize),
    Label,
    JumpIfEqual(usize),
    JumpIfNotEqual(usize),
    JumpIfGreater(usize),
    JumpIfSmaller(usize),
    JumpIfGreaterEqual(usize),
    JumpIfSmallerEqual(usize),
    ReturnValue,
    SendChannel,
    PopChannel,
    Spawn,
}

impl Instruction {
    pub fn from(instr_str: &[&str], labels: &Labels, variables: &Variables) -> Self {
        match instr_str {
            ["LOAD_VAL", val] => Instruction::LoadValue(val.parse::<isize>().unwrap()),
            ["WRITE_VAR", var_key] => {
                Instruction::WriteVariable(
                    *variables.get(
                        var_key.replace(&['\'', '"'][..], "").as_str()
                    )
                )
            },
            ["READ_VAR", var_key] => {
                Instruction::ReadVariable(
                    *variables.get(
                        var_key.replace(&['\'', '"'][..], "").as_str()
                    )
                )
            },
            ["ADD"] => Instruction::Add,
            ["SUB"] => Instruction::Sub,
            ["MULTIPLY"] => Instruction::Multiply,
            ["DIVIDE"] => Instruction::Divide,
            ["PRINT"] => Instruction::Print,
            ["PRINT", var_key] => {
                Instruction::PrintVariable(
                    var_key.replace(&['\'', '"'][..], ""),
                    *variables.get(
                        var_key.replace(&['\'', '"'][..], "").as_str()
                    )
                )
            },
            ["LABEL", _] => Instruction::Label,
            ["JUMP_IF_EQ", label_key] => Instruction::JumpIfEqual(*labels.get(label_key)),
            ["JUMP_IF_NQ", label_key] => Instruction::JumpIfNotEqual(*labels.get(label_key)),
            ["JUMP_IF_GR", label_key] => Instruction::JumpIfGreater(*labels.get(label_key)),
            ["JUMP_IF_SM", label_key] => Instruction::JumpIfSmaller(*labels.get(label_key)),
            ["JUMP_IF_GREQ", label_key] => Instruction::JumpIfGreaterEqual(*labels.get(label_key)),
            ["JUMP_IF_SMEQ", label_key] => Instruction::JumpIfSmallerEqual(*labels.get(label_key)),
            ["RETURN_VAL"] => Instruction::ReturnValue,
            ["SEND_CHANNEL"] => Instruction::SendChannel,
            ["POP_CHANNEL"] => Instruction::PopChannel,
            ["SPAWN"] => Instruction::Spawn,
            invalid_instr => panic!("Invalid instruction: {:?}", invalid_instr),
        }
    }
}
