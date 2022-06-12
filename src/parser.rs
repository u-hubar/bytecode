use crate::{instruction::Instruction, vm::Pointer, labels::Labels, variables::Variables};

pub type Bytecode<'buf> = Vec<Vec<&'buf str>>;
pub type Label = (String, Pointer);
pub type Variable = String;

pub struct Parser;

impl<'buf> Parser {
    pub fn parse_code(buffer: &'buf String) -> Bytecode<'buf> {
        buffer.split("\n")
            .map(|line| line.trim().split(" ").filter(|token| !token.is_empty()).collect::<Vec<_>>())
            .filter(|line_vec| !line_vec.is_empty())
            .collect::<Vec<_>>()
    }

    pub fn parse_labels(bytecode: &'buf Bytecode) -> Labels {
        bytecode.iter()
            .enumerate()
            .filter_map(|(ip, code_line)| Parser::find_label(code_line, ip))
            .collect::<Labels>()
    }

    pub fn parse_variables(bytecode: &'buf Bytecode) -> Variables {
        bytecode.iter()
            .filter_map(|code_line| Parser::find_variable(code_line))
            .collect::<Variables>()
    }

    pub fn parse_instructions(bytecode: &Bytecode, labels: &Labels, variables: &Variables) -> Vec<Instruction> {
        bytecode.iter()
            .map(|line| Instruction::from(line.as_slice(), labels, variables))
            .collect::<Vec<_>>()
    }

    fn find_label(code_line: &'buf [&str], ip: Pointer) -> Option<Label> {
        match code_line {
            ["LABEL", label_name] => Some((label_name.to_string(), ip)),
            _ => None,
        }
    }

    fn find_variable(code_line: &[&str]) -> Option<Variable> {
        match code_line {
            ["WRITE_VAR", var_name] => {
                Some(var_name.replace(&['\'', '"'][..], ""))
            },
            _ => None,
        }
    }
}
