use crate::lex::{Token, TokenType};

pub struct Representation {
    pub instructions: Vec<Instruction>,
}
// Change it according to standard
// Added 
impl Representation {
    pub fn new() -> Self {
        Representation { instructions: vec![] }
    } 

    pub fn parse(&mut self, tokens: Vec<Token>){
        let mut i = 0;
        let mut last =  TokenType::Eof;
        for token in tokens {
            let t =  token.token_type();
            
            if t == TokenType::Print {
                if i != 0 {
                    self.instructions.push(Instruction::Write((i-1) as *const u8));
                } 
                if i==0 {
                    
                }
            }
        }
    }

}

pub enum Instruction {
    Add(i64),
    Sub(i64),
    Read(*const u8),
    Write(*const u8),
    LoopStart(*const u8),
    LoopEnd(*const u8),
}
