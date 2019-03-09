
// lexing
// eg 1234 (x+y) translates into '1234' '(' 'x' '+' 'y' ')'

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

fn next_token(stream_iter : &mut std::iter::Peekable<std::str::Chars>) -> Option<Token> {

    let first = match stream_iter.next() {
        Some(x) => x,
        None => return None
    };

    match first {
        '0'..='9' => {
            let first_digit = first as u64 - ('0' as u64);
            let mut val : u64 = first_digit;

            loop {
                {
                    let peek = match stream_iter.peek() {
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
                stream_iter.next();
            }
            return Some(Token::Integer(val))
        },
        'a'..='z' | 'A'..='Z' | '_' => {
            let mut name = String::new();
            name.push(first);

            loop {
                {
                    let peek = match stream_iter.peek() {
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
                stream_iter.next();
            }
            return Some(Token::Name(name))
        },
        _ => return Some(Token::Symbol(first))
    };
}

fn lex_test() {
    let input = "+()1234+994abc";
    let mut iter = input.chars().peekable();
    loop  {
        let t = next_token(&mut iter);
        match t {
            Some(token) => print_token(token),
            None => break
        }
    }
}

fn main() {
    lex_test()
}
