
use crate::lex::{Token, TokenType};

pub struct SyntaxParser {
    stack: Vec<Token>,
}

impl SyntaxParser {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: &Vec<Token>) -> Result<(), String> {
        for token in tokens.iter() {
            
            match token.token_type() {
                TokenType::LoopStart => {
                    self.stack.push(token.clone());
                }
                TokenType::LoopEnd => {
                    if self.stack.is_empty() {
                        return Err("Syntax Error: Unmatched LoopEnd".to_string());
                    }
                    self.stack.pop();
                }
                _ => {}
            }
        }

        if !self.stack.is_empty() {
            return Err("Syntax Error: Unmatched LoopStart".to_string());
        }

        Ok(())
    }
}