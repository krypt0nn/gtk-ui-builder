use crate::ast::tree::Tree;
use crate::ast::entries::prelude::*;
use crate::ast::entry::Entry;

use super::tokenizer::Tokenizer;
use super::token::Token;
use super::parse_error::ParseError;

pub struct Parser;

impl Parser {
    pub fn parse<T: ToString>(text: T) -> Result<Tree, ParseError> {
        let mut tree = Tree::new();
        let text = text.to_string();

        let tokens = Tokenizer::parse(&text)?;
        let mut i = 0;

        while i < tokens.len() {
            // Use statements
            // 
            // using Adw 1.0
            if tokens[i].is_other_value("using") {
                if i + 2 < tokens.len() && tokens[i + 1].is_other() && tokens[i + 2].is_other() {
                    tree.require(Requirement::new(
                        tokens[i + 1].get_value().unwrap(),
                        tokens[i + 2].get_value().unwrap()
                    ));

                    i += 2;
                }

                else {
                    return Err(ParseError::IncorrectUseStatement {
                        message: format!("Incorrect use statement at offset {}", tokens[i].get_begin()),
                        offset: tokens[i].get_begin()
                    });
                }
            }

            // Components, events or properties definitions
            else if let Token::Other { value, .. } = &tokens[i] {
                let class = value.clone();

                // Property definition
                // 
                // property-name: property-value;
                if class.chars().last() == Some(':') {
                    if i + 1 < tokens.len() {
                        let mut j = i + 1;
                        let mut correct_ending = false;

                        while j < tokens.len() {
                            if let Token::Other { value, .. } = &tokens[j] {
                                if value.chars().last() == Some(';') {
                                    correct_ending = true;

                                    break;
                                }
                            }

                            j += 1;
                        }

                        if !correct_ending {
                            return Err(ParseError::IncorrectPropertyDefinition {
                                message: format!("Property value must be ended by semicolon, occured at offset {}", tokens[i].get_begin()),
                                offset: tokens[i].get_begin()
                            });
                        }

                        tree.add_child(Property::entry(class[..class.len() - 1].to_string(), {
                            if j - i <= 2 {
                                // [Text(..)] [Other(;)]  Example: "Hi";
                                if j - i == 2 {
                                    // TODO brackets support
                                    PropertyValue::Text(tokens[i + 1].get_value().unwrap())
                                }

                                // [Text(..;)] Example: Hi;
                                else {
                                    // TODO brackets support
                                    let text = tokens[i + 1].get_value().unwrap();
                                    
                                    PropertyValue::Text(text[..text.len() - 1].to_string())
                                }
                            } else {
                                let children = match Parser::parse(&text[tokens[i + 1].get_begin()..tokens[j].get_end()]) {
                                    Ok(tree) => tree,
                                    Err(err) => return Err(err.offset(tokens[i + 1].get_begin()))
                                };

                                let children = children.root.children;

                                if children.len() != 1 {
                                    return Err(ParseError::IncorrectPropertyDefinition {
                                        message: format!("Property value must be a single object, occured at offset {}", tokens[i].get_begin()),
                                        offset: tokens[i].get_begin()
                                    });
                                }

                                else {
                                    match &children[0] {
                                        Entry::Object(obj) => PropertyValue::Entry(obj.clone()),
                                        _ => {
                                            return Err(ParseError::IncorrectPropertyDefinition {
                                                message: format!("Property value must be an object, occured at offset {}", tokens[i].get_begin()),
                                                offset: tokens[i].get_begin()
                                            });
                                        }
                                    }
                                }
                            }
                        }));

                        i = j;
                    }

                    else {
                        return Err(ParseError::IncorrectPropertyDefinition {
                            message: format!("Incorrect property definition at offset {}", tokens[i].get_begin()),
                            offset: tokens[i].get_begin()
                        });
                    }
                }

                // Component event connection
                // 
                // clicked => { some code }
                else if i + 1 < tokens.len() && tokens[i + 1].is_other_value("=>") {
                    if i + 2 < tokens.len() {
                        if let Token::CurlyBrackets { begin, end, .. } = tokens[i + 2] {
                            // TODO: lazy brackets parsing. This will fix events definition errors

                            if cfg!(feature = "rhai-events") {
                                #[cfg(feature = "rhai-events")]
                                tree.add_child(RhaiEvent::entry(class, text[begin + 1..end].to_string()));

                                i += 2;
                            }

                            else {
                                return Err(ParseError::IncorrectEventDefinition {
                                    message: format!("Rhai feature is not enabled, occured at offset {}", tokens[i].get_begin()),
                                    offset: tokens[i].get_begin()
                                });
                            }
                        }

                        else {
                            return Err(ParseError::IncorrectEventDefinition {
                                message: format!("Undefined event value at offset {}", tokens[i].get_begin()),
                                offset: tokens[i].get_begin()
                            });
                        }
                    }

                    else {
                        return Err(ParseError::IncorrectEventDefinition {
                            message: format!("Incorrect event definition at offset {}", tokens[i].get_begin()),
                            offset: tokens[i].get_begin()
                        });
                    }
                }

                // Components definitions
                // 
                // Adw.ApplicationWindow {}
                // 
                // Adw.ApplicationWindow object_name {}
                else {
                    let mut name = None;

                    // Adw.ApplicationWindow
                    if i + 1 >= tokens.len() {
                        return Err(ParseError::IncorrectObjectDefinition {
                            message: format!("Incorrect object definition at offset {}", tokens[i].get_begin()),
                            offset: tokens[i].get_begin()
                        });
                    }

                    if let Token::Other { value, .. } = &tokens[i + 1] {
                        name = Some(value.clone());

                        i += 1;
                    }

                    // Adw.ApplicationWindow {}
                    if i + 1 < tokens.len() {
                        tree.add_child(Object::entry(class, name, {
                            if tokens[i + 1].get_end() - tokens[i + 1].get_begin() > 0 {
                                Self::parse(&text[tokens[i + 1].get_begin() + 1..tokens[i + 1].get_end()])?.root.children
                            } else {
                                Vec::new()
                            }
                        }));

                        i += 1;
                    }

                    // Adw.ApplicationWindow obj_name
                    else {
                        return Err(ParseError::IncorrectObjectDefinition {
                            message: format!("Incorrect object definition at offset {}", tokens[i].get_begin()),
                            offset: tokens[i].get_begin()
                        });
                    }
                }
            }

            // Random brackets or string in the middle of markup
            else {
                return Err(ParseError::IncorrectSyntax {
                    message: format!("Incorrect syntax at offset {}", tokens[i].get_begin()),
                    offset: tokens[i].get_begin()
                });
            }

            i += 1;
        }

        Ok(tree)
    }
}
