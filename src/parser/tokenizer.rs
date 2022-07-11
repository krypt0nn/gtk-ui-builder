use std::collections::VecDeque;

use super::token::Token;
use super::tokenize_error::TokenizeError;

pub struct Tokenizer;

impl Tokenizer {
    /// Parse syntax tokens from input string
    /// 
    /// ```
    /// use gtk_ui_builder::parser::prelude::*;
    /// 
    /// let tokens = Tokenizer::parse("[a b c]").expect("Failed to tokenize");
    /// 
    /// if let Token::SquareBrackets { tokens, .. } = &tokens[0] {
    ///     let mut list = Vec::new();
    /// 
    ///     for token in tokens {
    ///         if let Token::Other { value, .. } = token {
    ///             list.push(value);
    ///         }
    ///     }
    /// 
    ///     println!("Tokenized list: {:?}", list);
    /// }
    /// ```
    pub fn parse<T: ToString>(text: T) -> Result<Vec<Token>, TokenizeError> {
        let mut tokens = Vec::new();
        let mut word = String::new();

        let text = text.to_string().chars().collect::<Vec<char>>();

        let mut token_begin = 0;
        let mut i = 0;

        while i < text.len() {
            // Other token end
            if text[i].is_whitespace() {
                // Two ifs to avoid else execution
                if !word.is_empty() {
                    tokens.push(Token::Other {
                        begin: token_begin,
                        end: i - 1,
                        value: word
                    });

                    word = String::new();
                }

                token_begin = i + 1;
            }

            // Parse string
            else if text[i] == '"' {
                // Return Err if word is not empty (some_text"another_text)
                if !word.is_empty() {
                    return Err(TokenizeError::IncorrectChar {
                        message: format!("Incorrect character ({}) found at offset {}", text[i], i),
                        wrong_string: word,
                        offset: i
                    });
                }

                let mut correct_str = false;
                i += 1;

                while i < text.len() {
                    // TODO: slashes encoding
                    if text[i] == '"' {
                        correct_str = true;

                        break;
                    }

                    else {
                        word.push(text[i]);
                    }

                    i += 1;
                }

                // Return Err if string end wasn't found ("string)
                if !correct_str {
                    return Err(TokenizeError::IncorrectString {
                        message: format!("Incorrect string format found from offset {} to {}", token_begin, i),
                        begin: token_begin,
                        end: i,
                        wrong_string: word
                    });
                }

                tokens.push(Token::Text {
                    begin: token_begin,
                    end: i,
                    value: word
                });

                word = String::new();
                token_begin = i + 1;
            }

            // Parse brackets
            else if text[i] == '(' || text[i] == '[' || text[i] == '{' {
                // Return Err if word is not empty (some_text[another_text)
                if !word.is_empty() {
                    return Err(TokenizeError::IncorrectChar {
                        message: format!("Incorrect character ({}) found at offset {}", text[i], i),
                        wrong_string: word,
                        offset: i
                    });
                }

                let mut brackets_stack = VecDeque::from([text[i]]);
                let mut correct_order = false;

                i += 1;

                while i < text.len() {
                    if let Some(rev_bracket) = Self::get_rev_bracket(text[i]) {
                        if brackets_stack.back() == Some(&rev_bracket) {
                            brackets_stack.pop_back();

                            if brackets_stack.is_empty() {
                                correct_order = true;

                                break;
                            }
                        }

                        else {
                            brackets_stack.push_back(text[i]);
                        }
                    }

                    word.push(text[i]);
                    i += 1;
                }

                // Return Err if brackets end wasn't found ([string)
                if !correct_order {
                    return Err(TokenizeError::IncorrectBrackets {
                        message: format!("Incorrect brackets format found from offset {} to {}", token_begin, i),
                        begin: token_begin,
                        end: i,
                        wrong_string: word
                    });
                }

                let sub_tokens = Self::inc_tokens_offsets(Self::parse(word)?, token_begin + 1);

                tokens.push(match &text[token_begin] {
                    '(' => Token::Parentheses    { begin: token_begin, end: i, tokens: sub_tokens },
                    '[' => Token::SquareBrackets { begin: token_begin, end: i, tokens: sub_tokens },
                    '{' => Token::CurlyBrackets  { begin: token_begin, end: i, tokens: sub_tokens },
                    _ => unreachable!()
                });

                word = String::new();
                token_begin = i + 1;
            }

            // Push Other token character
            // There may be a situation like {}; where ; will be parsed as Other
            // while {} as CurlyBrackets, so technically ; will be the first character. This is wrong
            // so we need check previous character
            else if Self::is_normal_char(text[i], word.is_empty() && (if i > 0 { text[i - 1].is_whitespace() } else { true })) {
                word.push(text[i]);
            }

            // Wrong Other token character
            else {
                return Err(TokenizeError::IncorrectChar {
                    message: format!("Incorrect character ({}) found at offset {}", text[i], i),
                    wrong_string: word,
                    offset: i
                });
            }

            i += 1;
        }

        if !word.is_empty() {
            tokens.push(Token::Other {
                begin: token_begin,
                end: text.len() - 1,
                value: word
            });
        }

        Ok(tokens)
    }

    fn get_rev_bracket(bracket: char) -> Option<char> {
        match bracket {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),

            ')' => Some('('),
            ']' => Some('['),
            '}' => Some('{'),

            _ => None
        }
    }

    fn is_normal_char(char: char, first: bool) -> bool {
        (char >= 'a' && char <= 'z') ||
        (char >= 'A' && char <= 'Z') ||
        (char >= '0' && char <= '9') ||
        char == '.' || char == '_' ||
        (!first && (
            char == '-' || char == ';' || char == '(' ||
            char == ')' || char == '=' || char == '>' ||
            char == '<' || char == ':'
        ))
    }

    fn inc_tokens_offsets(mut tokens: Vec<Token>, offset: usize) -> Vec<Token> {
        for token in &mut tokens {
            match token {
                Token::Text  { begin, end, .. } => { *begin += offset; *end += offset; },
                Token::Other { begin, end, .. } => { *begin += offset; *end += offset; },

                Token::Parentheses { begin, end, tokens } => {
                    *begin += offset;
                    *end += offset;
                    *tokens = Self::inc_tokens_offsets(tokens.clone(), offset);
                },

                Token::SquareBrackets { begin, end, tokens } => {
                    *begin += offset;
                    *end += offset;
                    *tokens = Self::inc_tokens_offsets(tokens.clone(), offset);
                },

                Token::CurlyBrackets  { begin, end, tokens } => {
                    *begin += offset;
                    *end += offset;
                    *tokens = Self::inc_tokens_offsets(tokens.clone(), offset);
                }
            }
        }

        tokens
    }
}
