use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenizeError {
    IncorrectChar {
        message: String,
        wrong_string: String,
        offset: usize
    },
    IncorrectString {
        message: String,
        begin: usize,
        end: usize,
        wrong_string: String
    },
    IncorrectBrackets {
        message: String,
        begin: usize,
        end: usize,
        wrong_string: String
    }
}

impl TokenizeError {
    pub fn get_message(&self) -> &str {
        match self {
            Self::IncorrectChar     { message, .. } => message,
            Self::IncorrectString   { message, .. } => message,
            Self::IncorrectBrackets { message, .. } => message
        }.as_str()
    }

    pub fn offset(mut self, num: usize) -> Self {
        match &mut self {
            TokenizeError::IncorrectChar { offset, .. } => *offset += num,

            TokenizeError::IncorrectString { begin, end, .. } |
            TokenizeError::IncorrectBrackets { begin, end, .. } => {
                *begin += num;
                *end += num;
            }
        }

        self
    }
}

impl Into<Error> for TokenizeError {
    fn into(self) -> Error {
        Error::new(ErrorKind::Other, self.get_message())
    }
}
