
// lexing
// eg 1234 (x+y) translates into '1234' '(' 'x' '+' 'y' ')'

enum Token {
    Integer(u64),
}

fn next_token(s : &str) {
    
}

fn lex_test() {
    let input = "+()1234+994";
    next_token(&input);
}

fn main() {
    println!("Hello, world!");
    lex_test()
}
