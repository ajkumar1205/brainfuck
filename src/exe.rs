use std::io::Read;

use crate::lex::{Token, TokenType};

pub struct Runner {
    ins: usize,
    ptr: usize,
    tokens: Vec<Token>,
    stack: Vec<(usize, usize)>,
    tape: [u8; 30000],
}

impl Runner {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            ins: 0,
            ptr: 0,
            tokens,
            stack: Vec::new(),
            tape: [0u8; 30000],
        }
    }

    pub fn run(&mut self) {
        while self.ins < self.tokens.len() {
            let tk = &self.tokens[self.ins];
            match tk.token_type() {
                TokenType::Increment => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
                }
                TokenType::Decrement => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
                }
                TokenType::MoveRight => {
                    self.ptr = (self.ptr + 1) % 30000;
                }
                TokenType::MoveLeft => {
                    self.ptr = (self.ptr + 30000 - 1) % 30000;
                }
                TokenType::LoopStart => {
                    self.stack.push((self.ptr, self.ins));
                }
                TokenType::LoopEnd => {
                    if self.tape[self.stack.last().unwrap().0] == 0 {
                        self.stack.pop();
                    } else {
                        self.ins = self.stack.last().unwrap().1;
                    }
                }
                TokenType::Print => {
                    print!("{}", String::from_utf8_lossy(&[self.tape[self.ptr]]));
                }
                TokenType::Read => {
                    let mut buffer = vec![0];
                    std::io::stdin().read_exact(&mut buffer).unwrap();
                    self.tape[self.ptr] = buffer[0];
                }
                _ => {}
            }
            self.ins += 1;
        }
    }

    pub fn add(&mut self, tokens: &mut Vec<Token>) {
        self.tokens.append(tokens);
    }
}
