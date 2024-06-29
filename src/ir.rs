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
            match token.token_type() {
                TokenType::Increment if last == TokenType::Increment => {
                    i += 1;
                }
                _ => {}
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
