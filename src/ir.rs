use crate::lex::{Token, TokenType};

pub struct Representation {
    pub instructions: Vec<Instruction>,
}
// Change it according to standard
// Added
impl Representation {
    pub fn new() -> Self {
        Representation {
            instructions: vec![],
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        let mut count = 0;
        let mut ptr = 0;
        let mut last = TokenType::Eof;

        for token in tokens {
            
            match token.token_type() {
                TokenType::Print => {
                    self.instructions.push(Instruction::Print(ptr));
                }
                TokenType::Read => {
                    self.instructions.push(Instruction::Read(ptr));
                }
                TokenType::LoopStart => {
                    self.instructions.push(Instruction::LoopStart(ptr));
                }
                TokenType::LoopEnd => {
                    self.instructions.push(Instruction::LoopEnd(ptr));
                }
                TokenType::MoveLeft => {
                    ptr = (ptr + 30000 - 1) % 30000;
                    count = Representation::get();
                }
                TokenType::MoveRight => {
                    ptr = (ptr + 1) % 30000;
                }
                TokenType::Increment => {
                    count = count + 1;
                }
                TokenType::Decrement => {
                    count = count + 1;
                }
                _ => {}
            }
        }
    }

    // GET THE VALUE IN THE PREVIOUS INDEX 
    // SO THAT WE CAN MOVE LEFT RIGHT WITH 
    // CORRECT VALUES FOR SUMMATION
    fn get() -> i32 {
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
