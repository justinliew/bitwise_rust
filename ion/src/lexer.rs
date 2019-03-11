#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Integer(u64),
    Name(String),
    Symbol(char),
}

impl Token {
   fn is_token(&self, kind: &Token) -> bool {
       return self == kind
    }

    fn is_token_name(&self, name: &str) -> bool {
        match self {
            Token::Name(n) => n == name,
            _ => false 
        }
    }

    pub fn match_token(&self, kind: &Token, stream: &mut LexStream) -> bool {
        if self.is_token(kind) {
            stream.next_token();
            true
        } else {
            false
        }
    }

    pub fn expect_token(&self, kind: &Token, stream: &mut LexStream) -> bool {
        if self.is_token(kind) {
            stream.next_token();
            true
        } else {
            assert!(false, "Expected token {:?}, got {:?}", kind, self);
            false
        }
    }
}

pub fn print_token(t: Token) {
    match t {
        Token::Integer(i) => println!("Int: {}",i),
        Token::Name(n) => println!("Name: {}",n),
        Token::Symbol(s) => println!("Symbol: {}", s),
    }
}

pub struct LexStream<'a> {
    stream: String,
    stream_iter: std::iter::Peekable<std::str::Chars<'a>>,
    token: Option<Token>
}

impl<'a> LexStream<'a> {
    pub fn init(input: &str) -> LexStream {
        LexStream{
            stream: input.to_string(),
            stream_iter: input.chars().peekable(),
            token: None
        }
    }

    pub fn next_token(&mut self) {
        let first = match self.stream_iter.next() {
            Some(x) => x,
            None => {
                self.token = None;
                return
            }
        };

        match first {
            '0'..='9' => {
                let first_digit = first as u64 - ('0' as u64);
                let mut val : u64 = first_digit;

                loop {
                    {
                        let peek = match self.stream_iter.peek() {
                            Some(x) => x,
                            None => {
                                self.token = Some(Token::Integer(val));
                                return
                            }
                        };
                        if peek.is_digit(10) {
                            let digit = *peek as u64 - ('0' as u64);
                            val *= 10;
                            val += digit;                
                        } else {
                            break;
                        }
                    }
                    self.stream_iter.next();
                }
                self.token = Some(Token::Integer(val));
                return
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::new();
                name.push(first);

                loop {
                    {
                        let peek = match self.stream_iter.peek() {
                            Some(x) => x,
                            None => {
                                self.token = Some(Token::Name(name));
                                return
                            }
                        };

                        if peek.is_alphanumeric() {
                            name.push(*peek);
                        } else {
                            break;
                        }
                    }
                    self.stream_iter.next();
                }
                self.token = Some(Token::Name(name));
                return
            },
            _ => {
                self.token = Some(Token::Symbol(first));
                return
            }
        };
    }

    pub fn get_token(&self) -> Option<Token> {
        self.token.clone()
    }
}