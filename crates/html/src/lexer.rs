use core::slice::Iter;
use std::collections::VecDeque;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    TagBegin(String),
    TagEnd(String),
    Content(String),
    Attribute((String, String)),
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn lex(&mut self) -> Option<Vec<Token>> {
        let mut is_lexing_tag = false;

        let mut tokens = Vec::new();
        let mut iter = self.input.iter().peekable();
        loop {
            let it = iter.next();
            if it.is_none() {
                tokens.push(Token::EOF);
                break;
            }

            let ch = *it.unwrap();

            self.position += 1;

            // TODO: do not parse comments
            match ch {
                '<' => {
                    if let Some(&&next) = iter.peek() {
                        let is_close_tag = next == '/';

                        if next == '/' || next == '!' {
                            iter.next();
                        }

                        let element_name = get_next_word(&mut iter);

                        if is_close_tag {
                            tokens.push(Token::TagEnd(element_name));
                        } else {
                            tokens.push(Token::TagBegin(element_name.clone()));

                            is_lexing_tag = true;
                        }
                    }
                }
                '>' => is_lexing_tag = false,
                // handle tag names or attributes
                _ if ch.is_alphanumeric() || ch == '-' => {
                    // Collect alphanumeric strings as tags or text.
                    let mut value = String::new();
                    value.push(ch);

                    // if there is any current element then this must be an attribute
                    if is_lexing_tag {
                        // parse attribute name
                        while let Some(&&next) = iter.peek() {
                            if next.is_alphanumeric() || next == '-' {
                                value.push(iter.next().unwrap().clone());
                            } else {
                                break;
                            }
                        }

                        // parse attribute value if exists
                        let mut attr_value = String::new();
                        if let Some(&&next) = iter.peek() {
                            if next == '=' {
                                iter.next();

                                let mut quote_opened = false;

                                // parse the quote till it's ended
                                loop {
                                    match iter.next() {
                                        // some validation
                                        Some(&str_c) if str_c == '"' => {
                                            if quote_opened {
                                                break;
                                            }

                                            quote_opened = true;
                                        }
                                        Some(&str_c) if str_c == '\n' => return None,
                                        // parse the content
                                        Some(&str_c) => {
                                            attr_value.push(str_c);
                                        }
                                        None => return None,
                                    }
                                }
                            }
                        }

                        tokens.push(Token::Attribute((value, attr_value)));
                    } else {
                        // parse until the next element starts
                        while let Some(&&next) = iter.peek() {
                            if next == '<' {
                                break;
                            }

                            value.push(iter.next().unwrap().clone());
                        }

                        tokens.push(Token::Content(value));
                    }
                }

                _ => {}
            }
        }

        Some(tokens)
    }

    pub fn validate(tokens: &Vec<Token>) -> bool {
        let mut tags = VecDeque::new();
        for token in tokens {
            match token {
                Token::TagBegin(tag) if !Lexer::is_tag_self_closing(tag) => tags.push_back(tag),
                Token::TagEnd(tag) => {
                    if let Some(last_tag) = tags.pop_back() {
                        if last_tag != tag {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn is_tag_self_closing(tag: &str) -> bool {
        match tag.to_lowercase().as_str() {
            "doctype" | "br" | "hr" | "img" | "input" | "meta" | "link" => true,
            _ => false,
        }
    }
}

fn get_next_word(iter: &mut Peekable<Iter<char>>) -> String {
    let mut value = String::new();
    while let Some(&&next) = iter.peek() {
        if next.is_alphanumeric() || next == '-' {
            value.push(iter.next().unwrap().clone());
        } else {
            break;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = r#"
        <html>
        <h1>Hello, World!</h1>
        <div class="container" style="color: blue">
        </div>
        </html>
        "#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().expect("Failed to lex input");

        // Expected tokens
        let expected_tokens = vec![
            Token::TagBegin("html".to_string()),
            Token::TagBegin("h1".to_string()),
            Token::Content("Hello, World!".to_string()),
            Token::TagEnd("h1".to_string()),
            Token::TagBegin("div".to_string()),
            Token::Attribute(("class".to_string(), "container".to_string())),
            Token::Attribute(("style".to_string(), "color: blue".to_string())),
            Token::TagEnd("div".to_string()),
            Token::TagEnd("html".to_string()),
            Token::EOF,
        ];

        assert_eq!(tokens.len(), expected_tokens.len());

        for (i, (token, expected)) in tokens.iter().zip(expected_tokens.iter()).enumerate() {
            assert_eq!(token, expected, "Token mismatch at index {}", i);
        }
    }

    #[test]
    fn test_validate_correctly_nested_tags() {
        let tokens = vec![
            Token::TagBegin("html".to_string()),
            Token::TagBegin("body".to_string()),
            Token::TagBegin("h1".to_string()),
            Token::Content("Title".to_string()),
            Token::TagEnd("h1".to_string()),
            Token::TagEnd("body".to_string()),
            Token::TagEnd("html".to_string()),
            Token::EOF,
        ];

        assert!(Lexer::validate(&tokens), "Expected valid nesting of tags");
    }

    #[test]
    fn test_validate_incorrectly_nested_tags() {
        let tokens = vec![
            Token::TagBegin("html".to_string()),
            Token::TagBegin("body".to_string()),
            Token::TagBegin("h1".to_string()),
            Token::Content("Title".to_string()),
            Token::TagEnd("body".to_string()), // Incorrect closing tag
            Token::TagEnd("html".to_string()),
            Token::EOF,
        ];

        assert!(
            !Lexer::validate(&tokens),
            "Expected invalid nesting of tags due to incorrect closing tag"
        );
    }

    #[test]
    fn test_validate_unmatched_closing_tag() {
        let tokens = vec![
            Token::TagBegin("html".to_string()),
            Token::TagEnd("body".to_string()), // Unmatched closing tag
            Token::TagEnd("html".to_string()),
            Token::EOF,
        ];

        assert!(
            !Lexer::validate(&tokens),
            "Expected invalid nesting due to unmatched closing tag"
        );
    }

    #[test]
    fn test_validate_empty_tokens() {
        let tokens: Vec<Token> = vec![];
        assert!(
            Lexer::validate(&tokens),
            "Expected valid result for empty token list"
        );
    }
}
