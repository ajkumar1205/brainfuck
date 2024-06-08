use std::io::Read;

use crate::lex::{Token, TokenType};

pub struct Runner {
    ptr: usize,
    tokens: Vec<Token>,
    stack: Vec<i32>,
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
                    self.stack.push(i as i32);
                    self.handle_loop();
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
                _ => {}
            }
            i += 1;
        }
    }

    fn handle_loop(&mut self) {
        let val = self.stack.last().unwrap().clone() as usize;
        let mut i = val;
        while i < self.tokens.len() {
            if self.tape[self.ptr] == 0 {
                return;
            }
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
                    self.stack.push(i as i32);
                    self.handle_loop();
                }
                 TokenType::Print => {
                    print!("{}", self.tape[self.ptr] as char);
                }
                TokenType::Read => {
                    let mut buffer = vec![];
                    std::io::stdin().read_exact(&mut buffer).unwrap();
                    self.tape[self.ptr] = buffer[0];
                }
                TokenType::LoopEnd => {
                    i = val;
                }
                _ => {}
            }
            i += 1;
        }

        self.stack.pop();
    }
}
