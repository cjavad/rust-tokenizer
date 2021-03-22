#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Var(String),
    Str(String),
    Int(String),
    Add,
    Ass,
    START,
    IGNORE,
    EOL,
    EOF,
}

pub struct Parser {
    input_chars: Vec<char>,
    token_stack: Vec<Token>,
    cur_pos: usize,
    last_pos: usize,
    seek_str: bool,
}

impl Parser {
    pub fn new(input_str: String) -> Self {
        Parser {
            input_chars: input_str.chars().collect(),
            token_stack: vec![Token::START],
            cur_pos: 0,
            last_pos: 0,
            seek_str: false,
        }
    }

    pub fn next(&mut self) -> Token {
        if self.cur_pos >= self.input_chars.len() {
            return Token::EOF;
        }

        let old_pos = self.cur_pos.clone();
        let c = self.input_chars[self.cur_pos];
        self.cur_pos += 1;
        self.last_pos = old_pos.clone();

        match c {
            '+' => Token::Add,
            '=' => Token::Ass,
            ';' => Token::EOL,
            ' ' if !self.seek_str => Token::IGNORE,
            '"' => {
                if self.seek_str {
                    // end of string
                    self.seek_str = false;
                    Token::IGNORE
                } else {
                    self.seek_str = true;
                    Token::Str("".to_string())
                }
            }
            _ => {
                // variable or EOL
                let i = &self.token_stack.len() - 1;

                if self.seek_str {
                    match &self.token_stack[i] {
                        Token::Str(prev) => {
                            if self.seek_str {
                                self.token_stack[i] =
                                    Token::Str(prev.to_owned() + &c.clone().to_string());
                                Token::IGNORE
                            } else {
                                Token::IGNORE
                            }
                        }
                        _ => {
                            self.seek_str = false;
                            Token::IGNORE
                        }
                    }
                } else {
                    match &self.token_stack[i] {
                        Token::Int(prev) => {
                            self.token_stack[i] =
                                Token::Int(prev.to_owned() + &c.clone().to_string());
                            Token::IGNORE
                        }
                        Token::Var(prev) => {
                            self.token_stack[i] =
                                Token::Var(prev.to_owned() + &c.clone().to_string());
                            Token::IGNORE
                        }
                        _ => {
                            if c.is_numeric() {
                                // Is numeric
                                Token::Int(c.to_string())
                            } else {
                                // Is variable
                                Token::Var(c.to_string())
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let mut last_token: Token = Token::IGNORE;

        while last_token != Token::EOF {
            last_token = self.next();
            if last_token != Token::IGNORE {
                self.token_stack.push(last_token.clone());
            }
        }
    }
}

fn main() {
    let input_string = "x_a = 1 + 31".to_string();
    let mut parser = Parser::new(input_string.clone());
    parser.parse();

    println!("{:?}", input_string);

    for t in parser.token_stack {
        println!("{:?}", t);
    }
}
