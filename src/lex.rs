pub struct Lexer {
    input: String,
    current: usize,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            current: 0,
            tokens: Vec::new(),
            line: 1,
            col: 1,
        }
    }

    pub fn parse(&mut self) {
        while self.current < self.input.len() {
            let c = self.input.chars().nth(self.current).unwrap();
            match c {
                '+' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Increment,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '-' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Decrement,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '>' => {
                    self.tokens.push(Token {
                        token_type: TokenType::MoveRight,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '<' => {
                    self.tokens.push(Token {
                        token_type: TokenType::MoveLeft,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '[' => {
                    self.tokens.push(Token {
                        token_type: TokenType::LoopStart,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                ']' => {
                    self.tokens.push(Token {
                        token_type: TokenType::LoopEnd,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '.' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Print,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                ',' => {
                    self.tokens.push(Token {
                        token_type: TokenType::Read,
                        pos: self.current,
                        line: self.line,
                        col: self.col,
                    });
                    self.advance();
                }
                '/' => {
                    let ch = self.input.chars().nth(self.current + 1).unwrap();

                    if ch == '/' {
                        self.comment();
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.col = 1;
                    self.advance();
                }
                '\t' | ' ' | '\r' => {
                    self.advance();
                }
                _ => {
                    panic!(
                        "Unexpected character `{}` at col {} on line {}",
                        c, self.col, self.line
                    )
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            pos: self.current,
            line: self.line,
            col: self.col,
        });
    }

    fn advance(&mut self) {
        self.current += 1;
        self.col += 1;
    }

    fn comment(&mut self) {
        self.advance();
        while self.current < self.input.len() {
            let c = self.input.chars().nth(self.current).unwrap();
            if c == '\n' {
                self.line += 1;
                self.col = 1;
                break;
            }
            self.advance();
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    pos: usize,
    line: usize,
    col: usize,
}

impl Token {
    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// +
    Increment,
    /// -
    Decrement,
    /// >
    MoveRight,
    /// <
    MoveLeft,
    /// [
    LoopStart,
    /// ]
    LoopEnd,
    /// .
    Print,
    /// ,
    Read,
    /// End of file
    Eof,
}
