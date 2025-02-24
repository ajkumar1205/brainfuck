use crate::lex::{Token, TokenType};

pub struct Representation {
    pub instructions: Vec<Instruction>,
}

impl Representation {
    pub fn new() -> Self {
        Representation {
            instructions: vec![],
        }
    }

   pub fn parse(&mut self, tokens: &Vec<Token>) {
        let mut count = 0;
        let mut ptr = 0;

        for token in tokens {
            match token.token_type() {
                TokenType::Increment => {
                    count += 1;
                }
                TokenType::Decrement => {
                    count -= 1;
                }
                TokenType::Print => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0; // reset after flushing
                    }
                    self.instructions.push(Instruction::Print(ptr));
                }
                TokenType::Read => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0;
                    }
                    self.instructions.push(Instruction::Read(ptr));
                }
                TokenType::LoopStart => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0;
                    }
                    self.instructions.push(Instruction::LoopStart(ptr));
                }
                TokenType::LoopEnd => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0;
                    }
                    self.instructions.push(Instruction::LoopEnd(ptr));
                }
                TokenType::MoveLeft => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0;
                    }
                    ptr = (ptr + 30000 - 1) % 30000;
                }
                TokenType::MoveRight => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                        count = 0;
                    }
                    ptr = (ptr + 1) % 30000;
                }
                _ => {}
            }
        }
    }

    fn get(&self, ptr: usize) -> i32 {
        // Fix: Loop from end to start
        for i in (0..self.instructions.len()).rev() {
            if let Instruction::Sum(x, y) = self.instructions[i] {
                if ptr == y {
                    return x;
                }
            }
        } 
        0
    }
}

/// IR Instructions:
/// - Sum(value, ptr): Add/subtract value at memory position ptr
/// - Read(ptr): Read input byte into memory position ptr 
/// - Print(ptr): Output byte at memory position ptr
/// - LoopStart(ptr): Begin loop, continue if byte at ptr is non-zero
/// - LoopEnd(ptr): End loop, jump back to matching LoopStart if byte at ptr is non-zero

#[derive(Debug)]
pub enum Instruction {
    Sum(i32, usize),
    Read(usize),
    Print(usize),
    LoopStart(usize),
    LoopEnd(usize),
}
