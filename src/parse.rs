use std::process::exit;

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

    pub fn parse(&mut self) {
        for token in self.tokens.iter() {
            
            match token.token_type() {
                TokenType::LoopStart => {
                    self.stack.push(token.clone());
                }
                TokenType::LoopEnd => {
                    if self.stack.is_empty() {
                        eprintln!("Syntax Error: Unmatched LoopEnd");
                        exit(1);
                    }
                    self.stack.pop();
                }
                _ => {}
            }
        }

        if !self.stack.is_empty() {
            eprintln!("Syntax Error: Unmatched LoopStart");
            exit(1);
        }
    }
}