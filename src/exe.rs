use std::io::Read;

use crate::lex::{Token, TokenType};

pub struct Runner {
    ptr: usize,
    tokens: Vec<Token>,
    stack: Vec<(usize, usize)>,
    tape: [u8; 30000],
}

impl Runner {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            ptr: 0,
            tokens,
            stack: Vec::new(),
            tape: [0u8; 30000],
        }
    }

    pub fn run(&mut self) {
        let mut i = 0;
        while i < self.tokens.len() {
            let tk = &self.tokens[i];
            match tk.token_type() {
                TokenType::Increment => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
                }
                TokenType::Decrement => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
                }
                TokenType::MoveRight => {
                    self.ptr += 1;
                }
                TokenType::MoveLeft => {
                    self.ptr -= 1;
                }
                TokenType::LoopStart => {
                    self.stack.push((self.ptr, i));
                }
                TokenType::LoopEnd => {
                    if self.tape[self.stack.last().unwrap().0] == 0 {
                        self.stack.pop();
                    } else {
                        i = self.stack.last().unwrap().1;
                    }
                }
                TokenType::Print => {
                    print!("{}", self.tape[self.ptr] as char);
                }
                TokenType::Read => {
                    let mut buffer = vec![];
                    std::io::stdin().read_exact(&mut buffer).unwrap();
                    self.tape[self.ptr] = buffer[0];
                }
                TokenType::Eof => {
                    break;
                }
            }
            i += 1;
        }
    }

}
