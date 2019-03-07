
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

fn next_token(stream_iter : &mut std::str::Chars) -> Option<Token> {

    let mut next = match stream_iter.next() {
        Some(x) => x,
        None => return None
    };

    // TODO: I'm taking the approach from a C implementation of this
    // can we do this properly with just match, instead of the inner loop iterations?
    // also this consumes the character after an integer or name token since we are advancing the iterator, failing to match and then returning
    match next {
        '0'..='9' => {
            let mut val : u64 = 0;
            while next.is_digit(10) {
                let digit = next as u64 - ('0' as u64);
                val *= 10;
                val += digit;                
                next = match stream_iter.next() {
                    Some(x) => x,
                    None => {
                        return Some(Token::Integer(val))
                    }
                };
            }
            return Some(Token::Integer(val))
        },
        'a'..='z' | 'A'..='Z' | '_' => {
            let mut name = String::new();
            while next.is_alphanumeric() {
                name.push(next);
                next = match stream_iter.next() {
                    Some(x) => x,
                    None => {
                        return Some(Token::Name(name))
                    }
                };
            }
            return Some(Token::Name(name))
        },
        _ => return Some(Token::Symbol(next))
    };
}

fn lex_test() {
    let input = "+()1234+994abc";
    let mut iter = input.chars();
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
