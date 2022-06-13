use crate::{instruction::Instruction, vm::Pointer, labels::Labels, variables::Variables, functions::Functions, stack::Stack};

pub type Bytecode<'buf> = Vec<Vec<&'buf str>>;
pub type Label = (String, Pointer);
pub type Variable = String;
pub type Function = String;

pub struct Parser;

impl<'buf> Parser {
    pub fn parse_code(buffer: &'buf String) -> Bytecode<'buf> {
        buffer.split("\n")
            .map(|line| line.trim().split(" ").filter(|token| !token.is_empty()).collect::<Vec<_>>())
            .filter(|line_vec| !line_vec.is_empty())
            .collect::<Vec<_>>()
    }

    pub fn parse_functions(bytecode: &'buf Bytecode) -> Functions {
        let mut functions = Functions::new();
        let mut ip = 0;

        while ip < bytecode.len() {
            match bytecode[ip].as_slice() {
                ["FUNC", func_name] => {
                    let func_start_ip = ip;

                    while bytecode[ip] != &["RETURN"] && bytecode[ip] != &["RETURN_VAL"] {
                        ip += 1;
                    }

                    functions.insert(func_name.to_string(), (func_start_ip, ip));
                },
                _ => ip += 1,
            }
        }

        functions
    }

    pub fn parse_variables(bytecode: &'buf Bytecode) -> Variables<'buf> {
        let mut variables = Variables::new();
        let mut function_names = Stack::new();
        function_names.push("MAIN");

        for line in bytecode {
            match line.as_slice() {
                ["WRITE_VAR", var_name] |
                ["READ_VAR", var_name] |
                ["PRINT", var_name] => variables.insert_local(function_names.peek(), var_name),
                ["FUNC", func_name] => function_names.push(func_name),
                ["RETURN"] | ["RETURN_VAL"] => {function_names.pop();},
                _ => {},
            }
            println!("{:?} -> {:?}", line, variables);
        }

        variables
    }

    pub fn parse_labels(bytecode: &'buf Bytecode) -> Labels {
        bytecode.iter()
            .enumerate()
            .filter_map(|(ip, line)| Parser::find_label(line, ip))
            .collect::<Labels>()
    }

    pub fn parse_instructions(
        bytecode: &Bytecode,
        functions: &Functions,
        variables: &mut Variables,
        labels: &Labels,
    ) -> Vec<Instruction> {
        bytecode.iter()
            .map(
                |line|
                Instruction::from(
                    line.as_slice(), functions, variables, labels
                )
            )
            .collect::<Vec<_>>()
    }

    fn find_label(line: &'buf [&str], ip: Pointer) -> Option<Label> {
        match line {
            ["LABEL", label_name] => Some((label_name.to_string(), ip)),
            _ => None,
        }
    }
}
