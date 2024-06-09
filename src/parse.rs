
use crate::lex::{Token, TokenType};

pub struct SyntaxParser {
    tokens: Vec<Token>,
    stack: Vec<Token>,
}

impl SyntaxParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            stack: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        for token in self.tokens.iter() {
            
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