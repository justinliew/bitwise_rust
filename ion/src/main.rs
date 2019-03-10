#![allow(dead_code)]
#![allow(unused_variables)]
// TODO string interning

// Simple test Expression grammar:
// expr3 = INT | '(' expr ')' 
// expr2 = '-'? expr2 | expr3
// expr1 = expr2 ([/*] expr2)*
// expr0 = expr1 ([+-] expr1)*
// expr = expr0

fn parse_expr3(stream: &mut LexStream) {
    match stream.get_token() {
        Some(Token::Integer(n)) => {
            println!("Found int {}", n);
            stream.next_token();
        },
        Some(Token::Symbol(s)) => {
            if s == '(' {
                parse_expr(stream);
                stream.get_token().unwrap().expect_token(&Token::Symbol(')'),stream);
            } else {
            assert!(false, "Expected INT or '('; got {:?}", s)
            }
        },
        Some(n) => {
            assert!(false, "Expected INT or '('; got {:?}", n);
        }
        None => {
            assert!(false, "Expected INT or '('; got nothing");
        }
    };
}

fn parse_expr2(stream: &mut LexStream) {
    match stream.get_token() {
        Some(n) => {
            if n.match_token(&Token::Symbol('-'), stream) {
                parse_expr3(stream);
            } else {
                parse_expr3(stream);
            }
        },
        None => {}
    }
}

fn parse_expr1(stream: &mut LexStream) {
    parse_expr2(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '/' || s == '*' {
                    stream.next_token();
                    parse_expr2(stream);
                } else {
                    break;
                }
            },
            _ => {break;}
        }
    }
}

fn parse_expr0(stream: &mut LexStream) {
    parse_expr1(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '+' || s == '-' {
                    stream.next_token();
                    parse_expr1(stream);
                } else {
                    break;
                }
            },
            _ => {break;}
        }
    }
}

fn parse_expr(stream: &mut LexStream) {
    stream.next_token();
    parse_expr0(stream);
}

fn parse_test() {
    let input = "-3+4+(5*6)";
    let mut stream = LexStream::init(input);
    parse_expr(&mut stream);
}

#[derive(Debug,Clone,PartialEq)]
enum Token {
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

    fn match_token(&self, kind: &Token, stream: &mut LexStream) -> bool {
        if self.is_token(kind) {
            stream.next_token();
            true
        } else {
            false
        }
    }

    fn expect_token(&self, kind: &Token, stream: &mut LexStream) -> bool {
        if self.is_token(kind) {
            stream.next_token();
            true
        } else {
            assert!(false, "Expected token {:?}, got {:?}", kind, self);
            false
        }
    }
}

fn print_token(t: Token) {
    match t {
        Token::Integer(i) => println!("Int: {}",i),
        Token::Name(n) => println!("Name: {}",n),
        Token::Symbol(s) => println!("Symbol: {}", s),
    }
}

struct LexStream<'a> {
    stream: String,
    stream_iter: std::iter::Peekable<std::str::Chars<'a>>,
    token: Option<Token>
}

impl<'a> LexStream<'a> {
    fn init(input: &str) -> LexStream {
        LexStream{
            stream: input.to_string(),
            stream_iter: input.chars().peekable(),
            token: None
        }
    }

    fn next_token(&mut self) {
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

    fn get_token(&self) -> Option<Token> {
        self.token.clone()
    }
}

fn lex_test() {
    let input = "XY+(XY)_HELLO1,234+FOO!994";
    let mut stream = LexStream::init(input);
    loop  {
        stream.next_token();
        match stream.get_token() {
            Some(token) => print_token(token),
            None => break
        }
    }
}

fn main() {
    lex_test();
    parse_test();
}
