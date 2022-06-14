use crate::{labels::Labels, variables::{Variables, VariableAddress}, vm::Pointer, functions::Functions};

#[derive(Debug)]
pub enum Instruction {
    LoadValue(isize),
    WriteVariable(VariableAddress),
    ReadVariable(VariableAddress),
    Add,
    Sub,
    Multiply,
    Divide,
    Print,
    PrintVariable(String, VariableAddress),
    Jump(Pointer),
    JumpIfEqual(Pointer),
    JumpIfNotEqual(Pointer),
    JumpIfGreater(Pointer),
    JumpIfSmaller(Pointer),
    JumpIfGreaterEqual(Pointer),
    JumpIfSmallerEqual(Pointer),
    CallFunction(Pointer),
    Return,
    ReturnValue,
    SendChannel,
    PopChannel,
    Ignore,
}

impl Instruction {
    pub fn from(instr_str: &[&str], functions: &Functions, variables: &mut Variables, labels: &Labels) -> Self {
        match instr_str {
            ["LOAD_VAL", val] => Instruction::LoadValue(val.parse::<isize>().unwrap()),
            ["WRITE_VAR", _] => Instruction::WriteVariable(variables.pop_front().unwrap()),
            ["READ_VAR", _] => Instruction::ReadVariable(variables.pop_front().unwrap()),
            ["ADD"] => Instruction::Add,
            ["SUB"] => Instruction::Sub,
            ["MULTIPLY"] => Instruction::Multiply,
            ["DIVIDE"] => Instruction::Divide,
            ["PRINT"] => Instruction::Print,
            ["PRINT", var_name] => Instruction::PrintVariable(
                var_name.replace(&['\'', '"'][..], ""),
                variables.pop_front().unwrap(),
            ),
            ["LABEL", _] => Instruction::Ignore,
            ["FUNC", func_name] => Instruction::Jump(functions.get(func_name).unwrap().1),
            ["CALL", func_name] => Instruction::CallFunction(functions.get(func_name).unwrap().0),
            ["JUMP_IF_EQ", label_name] => Instruction::JumpIfEqual(*labels.get(label_name).unwrap()),
            ["JUMP_IF_NQ", label_name] => Instruction::JumpIfNotEqual(*labels.get(label_name).unwrap()),
            ["JUMP_IF_GR", label_name] => Instruction::JumpIfGreater(*labels.get(label_name).unwrap()),
            ["JUMP_IF_SM", label_name] => Instruction::JumpIfSmaller(*labels.get(label_name).unwrap()),
            ["JUMP_IF_GREQ", label_name] => Instruction::JumpIfGreaterEqual(*labels.get(label_name).unwrap()),
            ["JUMP_IF_SMEQ", label_name] => Instruction::JumpIfSmallerEqual(*labels.get(label_name).unwrap()),
            ["RETURN"] => Instruction::Return,
            ["RETURN_VAL"] => Instruction::ReturnValue,
            ["SEND_CHANNEL"] => Instruction::SendChannel,
            ["POP_CHANNEL"] => Instruction::PopChannel,
            invalid_instr => panic!("Invalid instruction: {:?}", invalid_instr),
        }
    }
}
