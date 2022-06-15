use crate::{labels::Labels, variables::{Variables, VariableAddress}, vm::Pointer, functions::Functions, errors::ParseError};

#[derive(Debug, PartialEq)]
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
    Ignore,
}

impl Instruction {
    pub fn from(
        instr_str: &[&str],
        functions: &Functions,
        variables: &mut Variables,
        labels: &Labels
    ) -> Result<Self, ParseError> {
        match instr_str {
            ["LOAD_VAL", val] => Ok(Instruction::LoadValue(val.parse::<isize>().unwrap())),
            ["WRITE_VAR", _] => Ok(Instruction::WriteVariable(variables.queue_pop_front().unwrap())),
            ["READ_VAR", _] => Ok(Instruction::ReadVariable(variables.queue_pop_front().unwrap())),
            ["ADD"] => Ok(Instruction::Add),
            ["SUB"] => Ok(Instruction::Sub),
            ["MULTIPLY"] => Ok(Instruction::Multiply),
            ["DIVIDE"] => Ok(Instruction::Divide),
            ["PRINT"] => Ok(Instruction::Print),
            ["PRINT", var_name] => Ok(
                Instruction::PrintVariable(
                    var_name.replace(&['\'', '"'][..], ""),
                    variables.queue_pop_front().unwrap(),
                )
            ),
            ["LABEL", _] => Ok(Instruction::Ignore),
            ["FUNC", func_name] => Ok(Instruction::Jump(functions.get(func_name).unwrap().1)),
            ["CALL", func_name] => Ok(Instruction::CallFunction(functions.get(func_name).unwrap().0)),
            ["JUMP_IF_EQ", label_name] => Ok(Instruction::JumpIfEqual(*labels.get(label_name).unwrap())),
            ["JUMP_IF_NQ", label_name] => Ok(Instruction::JumpIfNotEqual(*labels.get(label_name).unwrap())),
            ["JUMP_IF_GR", label_name] => Ok(Instruction::JumpIfGreater(*labels.get(label_name).unwrap())),
            ["JUMP_IF_SM", label_name] => Ok(Instruction::JumpIfSmaller(*labels.get(label_name).unwrap())),
            ["JUMP_IF_GREQ", label_name] => Ok(Instruction::JumpIfGreaterEqual(*labels.get(label_name).unwrap())),
            ["JUMP_IF_SMEQ", label_name] => Ok(Instruction::JumpIfSmallerEqual(*labels.get(label_name).unwrap())),
            ["RETURN"] => Ok(Instruction::Return),
            ["RETURN_VAL"] => Ok(Instruction::ReturnValue),
            invalid_instr => Err(ParseError::InvalidInstruction(invalid_instr.join(" "))),
        }
    }
}
