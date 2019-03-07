
// lexing
// eg 1234 (x+y) translates into '1234' '(' 'x' '+' 'y' ')'

enum Token {
    Integer(u64),
    Name(String),
}

fn print_token(t: Token) {
    match t {
        Token::Integer(i) => println!("{}",i),
        Token::Name(n) => println!("{}",n),
    }
}

fn next_token(stream_iter : &mut std::str::Chars) -> Option<Token> {

    let next = match stream_iter.next() {
        Some(x) => x,
        None => return None
    };

    let t = match next {
        '0'..='9' => {
            Some(Token::Integer(next as u64))
        },
        _ => {
            None
        }
    };
    t
}

fn lex_test() {
//    let input = "+()1234+994";
    let input = "1234567";
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
