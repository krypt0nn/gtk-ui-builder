use std::io::{Error, ErrorKind};

use super::tokenize_error::TokenizeError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    TokenizeError(TokenizeError),
    IncorrectUseStatement {
        message: String,
        offset: usize
    },
    IncorrectObjectDefinition {
        message: String,
        offset: usize
    },
    IncorrectPropertyDefinition {
        message: String,
        offset: usize
    },
    IncorrectSyntax {
        message: String,
        offset: usize
    }
}

impl ParseError {
    pub fn get_message(&self) -> &str {
        match self {
            Self::TokenizeError(err) => &err.get_message(),
            Self::IncorrectUseStatement { message, .. } => message.as_str(),
            Self::IncorrectObjectDefinition { message, .. } => message.as_str(),
            Self::IncorrectPropertyDefinition { message, .. } => message.as_str(),
            Self::IncorrectSyntax { message, .. } => message.as_str()
        }
    }

    pub fn offset(mut self, num: usize) -> Self {
        match &mut self {
            Self::TokenizeError(err) => return Self::TokenizeError(err.clone().offset(num)),

            ParseError::IncorrectUseStatement { offset, .. } |
            ParseError::IncorrectObjectDefinition { offset, .. } |
            ParseError::IncorrectPropertyDefinition { offset, .. } |
            ParseError::IncorrectSyntax { offset, .. } => *offset += num
        }

        self
    }
}

impl Into<Error> for ParseError {
    fn into(self) -> Error {
        Error::new(ErrorKind::Other, self.get_message())
    }
}

impl From<TokenizeError> for ParseError {
    fn from(err: TokenizeError) -> Self {
        Self::TokenizeError(err)
    }
}
