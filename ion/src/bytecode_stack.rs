use crate::lexer::LexStream;
use crate::lexer::Token;
use std::vec;

// we should be able to take an expression, convert it to bytecode, and then write the bytecode stack machine to run it and output the result.

const LIT : u32 = 1;
const ADD : u32 = 2;
const SUB : u32 = 3;
const MUL : u32 = 4;
const DIV : u32 = 5;
const NEG : u32 = 6;
const HALT : u32 = 7;

// expr3 = INT | '(' expr ')' 
// expr2 = '-'? expr2 | q120
// expr1 = expr2 ([/*] expr2)*
// expr0 = expr1 ([+-] expr1)*
// expr = expr0
// (1+2)*3

fn parse_expr3(stream: &mut LexStream) -> Vec<u32> {
    match stream.get_token() {
        Some(Token::Integer(n)) => {
            stream.next_token();
            vec![LIT,n as u32]
        },
        Some(Token::Symbol(s)) => {
            if s == '(' {
                let ret = parse_expr(stream);
                stream.get_token().unwrap().expect_token(&Token::Symbol(')'),stream);
                ret
            } else {
            assert!(false, "Expected INT or '('; got {:?}", s);
            vec![]
            }
        },
        Some(n) => {
            assert!(false, "Expected INT or '('; got {:?}", n);
            vec![]
        }
        None => {
            assert!(false, "Expected INT or '('; got nothing");
            vec![]
        }
    }
}

fn parse_expr2(stream: &mut LexStream) -> Vec<u32> {
    match stream.get_token() {
        Some(n) => {
            if n.match_token(&Token::Symbol('-'), stream) {
                let mut ret = parse_expr2(stream);
                ret.push(NEG);
                ret
            } else {
                parse_expr3(stream)
            }
        },
        None => vec![]
    }
}

fn parse_expr1(stream: &mut LexStream) -> Vec<u32> {
    let mut val = parse_expr2(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '/' || s == '*' {
                    stream.next_token();
                    let mut rval = parse_expr2(stream);
                    match s {
                        '/' => {
                            val.append(&mut rval);
                            val.push(DIV);
                        },
                        '*' => {
                            val.append(&mut rval);
                            val.push(MUL);
                        },
                        _ => {}
                    }
                } else {break;}
            },
            _ => {break;}
        }
    };
    val
}

fn parse_expr0(stream: &mut LexStream) -> Vec<u32> {
    let mut val = parse_expr1(stream);
    loop {
        match stream.get_token() {
            Some(Token::Symbol(s)) => {
                if s == '+' || s == '-' {
                    stream.next_token();
                    let mut rval = parse_expr1(stream);
                    match s {
                        '+' => {
                            val.append(&mut rval);
                            val.push(ADD);
                        },
                        '-' => {
                            val.append(&mut rval);
                            val.push(SUB);
                        }
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

fn parse_expr(stream: &mut LexStream) -> Vec<u32> {
    stream.next_token();
    parse_expr0(stream)
}

fn run_bytecode(code: &[u32]) -> u64 {
    42
}

// LIT x
// ADD
// SUB
// MUL
// DIV

fn gen_bytecode(stream: &mut LexStream) -> Vec<u32> {
    let res = parse_expr(stream);
    println!("gen_bytecode: {:?}", res);
    res
}

pub fn bytecode_stack_test() {
//    let input = "-3+4+(5*6)";
    let input = "-1+2";
    let mut stream = LexStream::init(input);

    let bytecode = gen_bytecode(&mut stream);
    let output = run_bytecode(&bytecode);
    println!("Calculated {}",output);
}