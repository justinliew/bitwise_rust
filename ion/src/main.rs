#![allow(dead_code)]
#![allow(unused_variables)]
// TODO string interning
// TODO more tests, better way of doing tests

// Simple test Expression grammar:
// expr3 = INT | '(' expr ')' 
// expr2 = '-'? expr2 | expr3
// expr1 = expr2 ([/*] expr2)*
// expr0 = expr1 ([+-] expr1)*
// expr = expr0

fn parse_expr3(stream: &mut LexStream) -> i64 {
    match stream.get_token() {
        Some(Token::Integer(n)) => {
            stream.next_token();
            n as i64
        },
        Some(Token::Symbol(s)) => {
            if s == '(' {
                let ret = parse_expr(stream);
                stream.get_token().unwrap().expect_token(&Token::Symbol(')'),stream);
                ret
            } else {
            assert!(false, "Expected INT or '('; got {:?}", s);
            0
            }
        },
        Some(n) => {
            assert!(false, "Expected INT or '('; got {:?}", n);
            0
        }
        None => {
            assert!(false, "Expected INT or '('; got nothing");
            0
        }
    }
}

fn parse_expr2(stream: &mut LexStream) -> i64 {
    match stream.get_token() {
        Some(n) => {
            if n.match_token(&Token::Symbol('-'), stream) {
                -parse_expr2(stream)
            } else {
                parse_expr3(stream)
            }
        },
        None => 0
    }
}

fn parse_expr1(stream: &mut LexStream) -> i64 {
    let mut val = parse_expr2(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '/' || s == '*' {
                    stream.next_token();
                    let rval = parse_expr2(stream);
                    match (s,rval) {
                        ('/',0) => assert!(false, "Cannot divide by 0"),
                        ('/',n) => val /= n,
                        ('*',_) => val *= rval,
                        (_,_) => {}
                    }
                } else {break;}
            },
            _ => {break;}
        }
    };
    val
}

fn parse_expr0(stream: &mut LexStream) -> i64 {
    let mut val = parse_expr1(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '+' || s == '-' {
                    stream.next_token();
                    let rval = parse_expr1(stream);
                    match s {
                        '+' => val += rval,
                        '-' => val -= rval,
                        _ => {}
                    }
                } else {
                    break;
                }
            },
            _ => {break;}
        }
    };
    val
}

fn parse_expr(stream: &mut LexStream) -> i64 {
    stream.next_token();
    parse_expr0(stream)
}

fn parse_test() {
    let input = "-3+4+(5*6)";
    let mut stream = LexStream::init(input);
    let out = parse_expr(&mut stream);
    println!("Calculated {}",out);

    let input2 = "(1)";
    let mut stream2 = LexStream::init(input2);
    let out2 = parse_expr(&mut stream2);
    println!("Calculated {}",out2);

    let input3 = "2*3+4*6";
    let mut stream3 = LexStream::init(input3);
    let out3 = parse_expr(&mut stream3);
    println!("Calculated {}",out3);
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
