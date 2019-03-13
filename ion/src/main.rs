#![allow(dead_code)]
#![allow(unused_variables)]

mod bytecode_stack;
mod lexer;

use lexer::LexStream;

// TODO string interning
// TODO more tests, better way of doing tests

pub fn lex_test() {
    println!("Lex Test 1");
    let input = "XY+(XY)_HELLO1,234+FOO!994";
    let mut stream = LexStream::init(input);
    loop  {
        stream.next_token();
        match stream.get_token() {
            Some(token) => lexer::print_token(token),
            None => break
        }
    }

    println!("Lex Test 2");
    let input2 = "10.2";
    let mut stream2 = LexStream::init(input2);
    loop  {
        stream2.next_token();
        match stream2.get_token() {
            Some(token) => lexer::print_token(token),
            None => break
        }
    }
}

fn main() {
    lex_test();

    use bytecode_stack::bytecode_stack_test;
    bytecode_stack_test();
}
