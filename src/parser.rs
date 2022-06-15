use crate::{instruction::Instruction, vm::Pointer, labels::Labels, variables::Variables, functions::Functions, stack::Stack, errors::ParseError};

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

    pub fn parse_functions(bytecode: &'buf Bytecode) -> Result<Functions, ParseError> {
        let mut functions = Functions::new();
        let mut ip = 0;

        while ip < bytecode.len() {
            match bytecode[ip].as_slice() {
                ["FUNC", func_name] => {
                    let func_start_ip = ip;
                    let mut actual_code_line = bytecode
                        .get(ip)
                        .ok_or(ParseError::FunctionNeverReturned(func_name.to_string()))?;

                    while actual_code_line != &["RETURN"] && actual_code_line != &["RETURN_VAL"] {
                        ip += 1;
                        actual_code_line = bytecode
                            .get(ip)
                            .ok_or(ParseError::FunctionNeverReturned(func_name.to_string()))?;
                    }

                    functions
                        .insert(func_name, (func_start_ip, ip))?;
                },
                _ => ip += 1,
            }
        }

        Ok(functions)
    }

    pub fn parse_variables(bytecode: &'buf Bytecode) -> Result<Variables<'buf>, ParseError> {
        let mut variables = Variables::new();
        let mut function_names = Stack::new();
        function_names.push("MAIN");

        for (i, line) in bytecode.iter().enumerate() {
            match line.as_slice() {
                ["WRITE_VAR", var_name] |
                ["READ_VAR", var_name] |
                ["PRINT", var_name] => {
                    variables.insert_local(
                        function_names
                            .peek()
                            .unwrap(),
                        var_name
                    )
                },
                ["FUNC", func_name] => function_names.push(func_name),
                ["RETURN"] | ["RETURN_VAL"] => {
                    if function_names.len() == 1 {
                        return Err(ParseError::ReturnOutsideFunction(i.to_string()))
                    }

                    function_names
                        .pop()
                        .unwrap();
                },
                _ => {},
            }
        }

        Ok(variables)
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
    ) -> Result<Vec<Instruction>, ParseError> {
        bytecode.iter()
            .map(
                |line|
                Instruction::from(line.as_slice(), functions, variables, labels)
            )
            .collect::<Result<Vec<_>, ParseError>>()
    }

    fn find_label(line: &'buf [&str], ip: Pointer) -> Option<Label> {
        match line {
            ["LABEL", label_name] => Some((label_name.to_string(), ip)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_code() {
        let buffer = "LOAD_VAL 5\nWRITE_VAR 'x'\nREAD_VAR 'x'".to_string();

        let actual_bytecode = Parser::parse_code(&buffer);
        let expected_bytecode = vec![
            vec!["LOAD_VAL", "5"],
            vec!["WRITE_VAR", "'x'"],
            vec!["READ_VAR", "'x'"],
        ];

        assert_eq!(actual_bytecode, expected_bytecode);
        
    }

    #[test]
    fn parse_functions() {
        let bytecode = vec![
            vec!["LOAD_VAL", "5"],
            vec!["WRITE_VAR", "'x'"],
            vec!["READ_VAR", "'x'"],
            vec!["FUNC", "TEST1"],
            vec!["LOAD_VAL", "8"],
            vec!["PRINT"],
            vec!["RETURN"],
            vec!["FUNC", "TEST2"],
            vec!["LOAD_VAL", "10"],
            vec!["PRINT"],
            vec!["RETURN"],
        ];

        let actual_functions = Parser::parse_functions(&bytecode).unwrap();

        let mut expected_functions = Functions::new();
        expected_functions.insert("TEST1", (3, 6)).unwrap();
        expected_functions.insert("TEST2", (7, 10)).unwrap();

        assert_eq!(actual_functions, expected_functions);
    }

    #[test]
    fn parse_functions_should_return_error_for_duplicated_functions() {
        let bytecode = vec![
            vec!["FUNC", "TEST1"],
            vec!["LOAD_VAL", "8"],
            vec!["PRINT"],
            vec!["RETURN"],
            vec!["FUNC", "TEST1"],
            vec!["LOAD_VAL", "10"],
            vec!["PRINT"],
            vec!["RETURN"],
        ];

        let functions = Parser::parse_functions(&bytecode);

        assert!(functions.is_err());
    }

    #[test]
    fn parse_functions_should_return_error_when_function_is_never_returned() {
        let bytecode = vec![
            vec!["FUNC", "TEST1"],
            vec!["LOAD_VAL", "8"],
            vec!["PRINT"],
        ];

        let functions = Parser::parse_functions(&bytecode);

        assert!(functions.is_err());
    }

    #[test]
    fn parse_variables() {
        let bytecode = vec![
            vec!["LOAD_VAL", "5"],
            vec!["WRITE_VAR", "'x'"],
            vec!["FUNC", "TEST1"],
            vec!["LOAD_VAL", "8"],
            vec!["WRITE_VAR", "'x'"],
            vec!["RETURN"],
            vec!["FUNC", "TEST2"],
            vec!["LOAD_VAL", "10"],
            vec!["WRITE_VAR", "'z'"],
            vec!["RETURN"],
            vec!["READ_VAR", "'x'"],
            vec!["WRITE_VAR", "'y'"]
        ];

        let actual_variables = Parser::parse_variables(&bytecode).unwrap();

        let mut expected_variables = Variables::new();
        expected_variables.insert_local("MAIN", "'x'");
        expected_variables.insert_local("TEST1", "'x'");
        expected_variables.insert_local("TEST2", "'z'");
        expected_variables.insert_local("MAIN", "'x'");
        expected_variables.insert_local("MAIN", "'y'");

        assert_eq!(actual_variables, expected_variables);
    }

    #[test]
    fn parse_variables_should_return_error_when_return_from_non_existent_function() {
        let bytecode = vec![
            vec!["RETURN"],
        ];

        let variables = Parser::parse_variables(&bytecode);

        assert!(variables.is_err());
    }

    #[test]
    fn parse_labels() {
        let bytecode = vec![
            vec!["LOAD_VAL", "5"],
            vec!["WRITE_VAR", "'x'"],
            vec!["LABEL", "LOOP"],
            vec!["READ_VAR", "'x'"],
            vec!["LOAD_VAL", "1"],
            vec!["ADD"],
            vec!["WRITE_VAR", "'x'"],
            vec!["READ_VAR", "'x'"],
            vec!["LOAD_VAL", "10"],
            vec!["JUMP_IF_SM", "LOOP"],
        ];

        let actual_labels = Parser::parse_labels(&bytecode);

        let mut expected_labels = Labels::new();
        expected_labels.insert("LOOP", 2).unwrap();

        assert_eq!(actual_labels, expected_labels);
    }

    #[test]
    #[should_panic(expected = "Label duplicate found during parsing.")]
    fn parse_labels_should_panic_for_duplicated_labels() {
        let bytecode = vec![
            vec!["LABEL", "LOOP"],
            vec!["LABEL", "LOOP"],
        ];

        Parser::parse_labels(&bytecode);
    }

    #[test]
    fn parse_instructions() {
        let bytecode = vec![
            vec!["LOAD_VAL", "5"],
            vec!["WRITE_VAR", "'x'"],
            vec!["LABEL", "LOOP"],
            vec!["READ_VAR", "'x'"],
            vec!["LOAD_VAL", "1"],
            vec!["ADD"],
            vec!["WRITE_VAR", "'x'"],
            vec!["READ_VAR", "'x'"],
            vec!["LOAD_VAL", "10"],
            vec!["JUMP_IF_SM", "LOOP"],
        ];

        let functions = Parser::parse_functions(&bytecode).unwrap();
        let mut variables = Parser::parse_variables(&bytecode).unwrap();
        let labels = Parser::parse_labels(&bytecode);

        let actual_instructions = Parser::parse_instructions(
            &bytecode,
            &functions,
            &mut variables,
            &labels
        ).unwrap();

        let expected_instructions = vec![
            Instruction::LoadValue(5),
            Instruction::WriteVariable(0),
            Instruction::Ignore,
            Instruction::ReadVariable(0),
            Instruction::LoadValue(1),
            Instruction::Add,
            Instruction::WriteVariable(0),
            Instruction::ReadVariable(0),
            Instruction::LoadValue(10),
            Instruction::JumpIfSmaller(2),
        ];

        assert_eq!(actual_instructions, expected_instructions);
    }

    #[test]
    fn parse_instructions_should_return_error_for_non_existent_instruction() {
        let bytecode = vec![
            vec!["TEST", "INSTRUCTION"],
        ];

        let functions = Parser::parse_functions(&bytecode).unwrap();
        let mut variables = Parser::parse_variables(&bytecode).unwrap();
        let labels = Parser::parse_labels(&bytecode);

        let instructions = Parser::parse_instructions(
            &bytecode,
            &functions,
            &mut variables,
            &labels
        );

        assert!(instructions.is_err());
    }
}

