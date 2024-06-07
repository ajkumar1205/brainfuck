use crate::lex::{Token, TokenType};

pub struct Runner {
    tokens: Vec<Token>,
}

impl Runner {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn run(&mut self) {
        let mut tape = [0u8; 30000];
        let mut ptr = 0;
        let mut stack: Vec<i32> = Vec::new();

        let mut i = 0;
        for tk in self.tokens.iter() {
            match tk.token_type() {
                TokenType::Increment => {
                    tape[ptr] = tape[ptr].wrapping_add(1);
                }
                TokenType::Decrement => {
                    tape[ptr] = tape[ptr].wrapping_sub(1);
                }
                TokenType::MoveRight => {
                    ptr += 1;
                }
                TokenType::MoveLeft => {
                    ptr -= 1;
                }
                TokenType::LoopStart => {
                    stack.push(i);
                }
                _ => {}
            }
        }
    }
}