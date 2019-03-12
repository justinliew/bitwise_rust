fn syntax_error(s : String) {
    println!("Syntax error: {}", s);
}

#[derive(Debug,Clone,PartialEq)]
pub enum TokenMod
{
    Dec,
    Hex,
    Bin,
    Oct,
}

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Integer(u64, TokenMod),
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
        Token::Integer(i,_) => println!("Int: {}",i),
        Token::Name(n) => println!("Name: {}",n),
        Token::Symbol(s) => println!("Symbol: {}", s),
    }
}

fn is_valid_digit() -> bool {
    false
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

    fn scan_int(&mut self, first: char) -> (u64, TokenMod) {
        let first_digit = first as u8 - ('0' as u8);
        let mut val : u64 = first_digit as u64;

        // 0x123456789abcdef
        let mut base = 10;
        let mut token_mod = TokenMod::Dec;
        if  first_digit == 0 {
            let cur = match self.stream_iter.next() {
                Some(x) => x,
                None => {self.token = None; return (val,token_mod);}
            }.to_lowercase().to_string();
            if cur == "x" {
                base = 16;
                token_mod = TokenMod::Hex;
            } else if cur == "b" {
                base = 2;
                token_mod = TokenMod::Bin;
            } else {
                base = 8;
                token_mod = TokenMod::Oct;
            }
        }

        loop {
            {
                let peek = match self.stream_iter.peek() {
                    Some(x) => x,
                    None => {
                        return (val,token_mod)
                    }
                };
                if peek.is_digit(10) {
                    let digit = *peek as u64 - ('0' as u64);
                    if val > (std::u64::MAX - digit)/10 {
                        syntax_error(format!("Integer overflow: {}", val));
                        // TODO skip over remaining digits
                    }
                    val *= 10;
                    val += digit;                
                } else {
                    break;
                }
            }
            self.stream_iter.next();
        }
        (val,token_mod)
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
                let (val,base) = self.scan_int(first);
                self.token = Some(Token::Integer(val,base));
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