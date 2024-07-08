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
                TokenType::Print => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    self.instructions.push(Instruction::Print(ptr));
                }
                TokenType::Read => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    self.instructions.push(Instruction::Read(ptr));
                }
                TokenType::LoopStart => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    self.instructions.push(Instruction::LoopStart(ptr));
                }
                TokenType::LoopEnd => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    self.instructions.push(Instruction::LoopEnd(ptr));
                }
                TokenType::MoveLeft => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    ptr = (ptr + 30000 - 1) % 30000;
                    count = self.get(ptr);
                }
                TokenType::MoveRight => {
                    if count != 0 {
                        self.instructions.push(Instruction::Sum(count, ptr));
                    }
                    ptr = (ptr + 1) % 30000;
                    count = self.get(ptr);
                }
                TokenType::Increment => {
                    count = count + 1;
                }
                TokenType::Decrement => {
                    count = count - 1;
                }
                _ => {}
            }
        }
    }

    fn get(&self, ptr: usize) -> i32 {
        for i in self.instructions.len()-1..0 {
            if let Instruction::Sum(x, y) = self.instructions[i] {
                if ptr == y {
                    return x;
                }
            }
        } 
        0
    }
}

pub enum Instruction {
    Sum(i32, usize),
    Read(usize),
    Print(usize),
    LoopStart(usize),
    LoopEnd(usize),
}
