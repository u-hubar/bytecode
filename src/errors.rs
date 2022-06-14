use std::{error::Error, fmt::{Display, Formatter, Debug}, fmt::Result as FmtResult};

pub enum ParseError {
    DuplicatedFunction,
    FunctionNotFound,
    VariableNotFound,
    DuplicatedLabel,
    LabelNotFound,
    FunctionNeverReturned,
    ReturnOutsideFunction,
    InvalidInstruction,
}

pub enum RuntimeError {
    EmptyStack,
    WrongStackIndex,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::DuplicatedFunction => "Function duplicate found during parsing",
            Self::FunctionNotFound => "Function has never been declared.",
            Self::VariableNotFound => "Variable has never been written.",
            Self::DuplicatedLabel => "Label duplicate found during parsing.",
            Self::LabelNotFound => "Jump to non-existant label found.",
            Self::ReturnOutsideFunction => "Return can only be used within a function.",
            Self::FunctionNeverReturned => "Missing return statement in the function.",
            Self::InvalidInstruction => "Unknown instruction.",
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
