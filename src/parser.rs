use crate::{instruction::Instruction, vm::Pointer, labels::Labels};

pub type Bytecode<'buf> = Vec<Vec<&'buf str>>;
pub type Label<'buf> = (&'buf str, Pointer);

pub struct Parser;

impl<'buf> Parser {
    pub fn parse_code(buffer: &'buf String) -> Bytecode<'buf> {
        buffer.split("\n")
            .map(|line| line.trim().split(" ").filter(|token| !token.is_empty()).collect::<Vec<_>>())
            .filter(|line_vec| !line_vec.is_empty())
            .collect::<Vec<_>>()
    }

    pub fn parse_labels(bytecode: &'buf Bytecode) -> Labels<'buf> {
        bytecode.iter()
            .enumerate()
            .filter_map(|(ip, code_line)| Parser::find_label(code_line, ip))
            .collect::<Labels>()
    }

    pub fn parse_instructions(bytecode: &Bytecode) -> Vec<Instruction> {
        bytecode.iter()
            .map(|line| Instruction::from(line.as_slice()))
            .collect::<Vec<_>>()
    }

    fn find_label(code_line: &'buf [&str], ip: Pointer) -> Option<Label<'buf>> {
        match code_line {
            ["LABEL", label_name] => Some((label_name, ip)),
            _ => None,
        }
    }
}
