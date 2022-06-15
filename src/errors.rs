use std::{error::Error, fmt::{Display, Formatter, Debug}, fmt::Result as FmtResult};

pub enum ParseError {
    DuplicatedFunction(String),
    FunctionNotFound(String),
    VariableNotFound,
    DuplicatedLabel(String),
    LabelNotFound(String),
    FunctionNeverReturned(String),
    ReturnOutsideFunction(String),
    InvalidInstruction(String),
}

pub enum RuntimeError {
    EmptyStack,
    WrongStackIndex,
}

impl ParseError {
    fn message(&self) -> String {
        match self {
            Self::DuplicatedFunction(func_name) => format!(
                "Function '{}' duplicate found during parsing", func_name
            ),
            Self::FunctionNotFound(func_name) => format!(
                "Function '{}' has never been declared.", func_name
            ),
            Self::VariableNotFound => "Variable has never been initialized.".to_string(),
            Self::DuplicatedLabel(label_name) => format!(
                "Label '{}' duplicate found during parsing.", label_name
            ),
            Self::LabelNotFound(label_name) => format!(
                "Jump to non-existant label '{}' found.", label_name
            ),
            Self::ReturnOutsideFunction(instr_num) => format!(
                "Return can only be used within a function (Line #{}).", instr_num
            ),
            Self::FunctionNeverReturned(func_name) => format!(
                "Missing return statement in the function '{}'.", func_name
            ),
            Self::InvalidInstruction(instr) => format!(
                "Unknown instruction '{}'.", instr
            ),
        }
    }
}

impl RuntimeError {
    fn message(&self) -> &str {
        match self {
            Self::EmptyStack => "Stack is empty, nothing to pop/peek.",
            Self::WrongStackIndex => "Element with provided index doesn't exist in the stack.",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error  for ParseError {}

impl Error  for RuntimeError {}
