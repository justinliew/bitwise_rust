// TODO string interning

#[derive(Debug)]
enum Token {
    Integer(u64),
    Name(String),
    Symbol(char),
}

fn print_token(t: Token) {
    match t {
        Token::Integer(i) => println!("Int: {}",i),
        Token::Name(n) => println!("Name: {}",n),
        Token::Symbol(s) => println!("Symbol: {}", s),
    }
}

// we should create a single "current token"

fn is_token(kind: Token) -> bool {
    // token.kind == kind
    false
}

fn is_token_name(name: &str) -> bool {
    // token.kind == TOKEN_NAME && token.name == name
    false
}

fn match_token(kind: Token) -> bool {
    // if is_token(kind) then next_token() return true; else return false
    false
}

fn expect_token(kind: Token) -> bool {
    // if is_token(kind) then next_token() return true; else return false
    // difference is we assert/error if we don't match
    false
}

struct LexStream<'a> {
    stream: String,
    stream_iter: std::iter::Peekable<std::str::Chars<'a>>
}

impl<'a> LexStream<'a> {
    fn init(input: &str) -> LexStream {
        LexStream{
            stream: input.to_string(),
            stream_iter: input.chars().peekable()}
    }

    //stream_iter : &mut std::iter::Peekable<std::str::Chars>
    fn next_token(&mut self) -> Option<Token> {

        let first = match self.stream_iter.next() {
            Some(x) => x,
            None => return None
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
                                return Some(Token::Integer(val));
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
                return Some(Token::Integer(val))
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::new();
                name.push(first);

                loop {
                    {
                        let peek = match self.stream_iter.peek() {
                            Some(x) => x,
                            None => {
                                return Some(Token::Name(name))
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
                return Some(Token::Name(name))
            },
            _ => return Some(Token::Symbol(first))
        };
    }
}

fn lex_test() {
    let input = "XY+(XY)_HELLO1,234+FOO!994";
    let mut stream = LexStream::init(input);
    loop  {
        let t = stream.next_token();
        match t {
            Some(token) => print_token(token),
            None => break
        }
    }
}

fn main() {
    lex_test()
}
