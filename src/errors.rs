use std::{error::Error, fmt::{Display, Formatter, Debug}, fmt::Result as FmtResult};

pub enum ParseError {
    DuplicatedFunction,
    FunctionNotFound,
    VariableNotFound,
    DuplicatedLabel,
    LabelNotFound,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::DuplicatedFunction => "Function duplicate found during parsing",
            Self::FunctionNotFound => "Function has never been declared.",
            Self::VariableNotFound => "Variable has never been written.",
            Self::DuplicatedLabel => "Label duplicate found during parsing.",
            Self::LabelNotFound => "Jump to non-existant label found.",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error  for ParseError {}
