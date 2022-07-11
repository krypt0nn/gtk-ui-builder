#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// `"Example string"`
    Text {
        begin: usize,
        end: usize,
        value: String
    },
    
    /// `(Example "parentheses")`
    Parentheses {
        begin: usize,
        end: usize,
        tokens: Vec<Token>
    },

    // TODO:
    
    /// `[Example, "square brackets"]`
    SquareBrackets {
        begin: usize,
        end: usize,
        tokens: Vec<Token>
    },

    /// `{example: "curly brackets", a: b}`
    CurlyBrackets {
        begin: usize,
        end: usize,
        tokens: Vec<Token>
    },

    /// `example values`
    Other {
        begin: usize,
        end: usize,
        value: String
    }
}

impl Token {
    pub fn is_other(&self) -> bool {
        match self {
            Self::Other { .. } => true,
            _ => false
        }
    }

    pub fn is_other_value<T: ToString>(&self, other: T) -> bool {
        match self {
            Self::Other { value, .. } => *value == other.to_string(),
            _ => false
        }
    }

    pub fn get_value(&self) -> Option<String> {
        match self {
            Token::Text  { value, .. } |
            Token::Other { value, .. } => Some(value.clone()),
            _ => None
        }
    }

    pub fn get_begin(&self) -> usize {
        match *self {
            Token::Text           { begin, .. } => begin,
            Token::Parentheses    { begin, .. } => begin,
            Token::SquareBrackets { begin, .. } => begin,
            Token::CurlyBrackets  { begin, .. } => begin,
            Token::Other          { begin, .. } => begin
        }
    }

    pub fn get_end(&self) -> usize {
        match *self {
            Token::Text           { end, .. } => end,
            Token::Parentheses    { end, .. } => end,
            Token::SquareBrackets { end, .. } => end,
            Token::CurlyBrackets  { end, .. } => end,
            Token::Other          { end, .. } => end
        }
    }
}
