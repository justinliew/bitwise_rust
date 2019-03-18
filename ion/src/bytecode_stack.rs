use crate::lexer::LexStream;
use crate::lexer::Token;
use std::vec;

// we should be able to take an expression, convert it to bytecode, and then write the bytecode stack machine to run it and output the result.

const LIT : u64 = 1;
const ADD : u64 = 2;
const SUB : u64 = 3;
const MUL : u64 = 4;
const DIV : u64 = 5;
const NEG : u64 = 6;
const HALT : u64 = 7;

// expr3 = INT | '(' expr ')' 
// expr2 = '-'? expr2 | q120
// expr1 = expr2 ([/*] expr2)*
// expr0 = expr1 ([+-] expr1)*
// expr = expr0
// (1+2)*3

fn parse_expr3(stream: &mut LexStream) -> Vec<u64> {
    match stream.get_token() {
        Some(Token::Integer(n,_)) => {
            stream.next_token();
            vec![LIT,n as u64]
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

fn parse_expr2(stream: &mut LexStream) -> Vec<u64> {
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

fn parse_expr1(stream: &mut LexStream) -> Vec<u64> {
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

fn parse_expr0(stream: &mut LexStream) -> Vec<u64> {
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

fn parse_expr(stream: &mut LexStream) -> Vec<u64> {
    stream.next_token();
    parse_expr0(stream)
}

/*
const LIT : u64 = 1;
const ADD : u64 = 2;
const SUB : u64 = 3;
const MUL : u64 = 4;
const DIV : u64 = 5;
const NEG : u64 = 6;
const HALT : u64 = 7;

*/
fn run_bytecode(code: &[u64]) -> u64 {
    println!("Bytecode stream: {:?}", code);
    let mut iter = code.iter();
    let mut stack : Vec<u64> = vec![];

    loop {
        let v = match iter.next() {
            Some(x) => x,
            None => {
                if stack.len() == 1 {
                    return *stack.first().unwrap()
                } else {
                    &HALT
                }
            }
        };

        match *v {
            LIT => {
                let literal = match iter.next() {
                    Some(x) => x,
                    None => {
                        assert!(false, "malformed bytecode; missing a literal after a LIT op"); 
                        return 0;
                    }
                };
                stack.push(*literal);
            },
            ADD => {
                assert!(stack.len()>=2, "malformed bytecode; not enough operands for ADD op");
                let lval = stack.pop().unwrap();
                let rval = stack.pop().unwrap();
                stack.push(lval + rval);
            },
            SUB => {
                assert!(stack.len()>=2, "malformed bytecode; not enough operands for SUB op");
                let lval = stack.pop().unwrap();
                let rval = stack.pop().unwrap();
                stack.push(rval - lval);
            },
            MUL => {
                assert!(stack.len()>=2, "malformed bytecode; not enough operands for MUL op");
                let lval = stack.pop().unwrap();
                let rval = stack.pop().unwrap();
                stack.push(rval * lval);
            },
            DIV => {
                assert!(stack.len()>=2, "malformed bytecode; not enough operands for DIV op");
                let lval = stack.pop().unwrap();
                let rval = stack.pop().unwrap();
                assert!(rval == 0, "cannot divide by 0");
                stack.push(rval / lval);
            },
            NEG => {
                // how do we handle negative numbers?
                // assert!(stack.len()>=1, "malformed bytecode; not enough operands for SUB op");
                // let val = stack.pop().unwrap();
                // stack.push(-val);
            },
            HALT => {

            },
            _ => {}
        }
    }
}

fn gen_bytecode(stream: &mut LexStream) -> Vec<u64> {
    parse_expr(stream)
}

pub fn bytecode_stack_test() {
    println!("Bytecode evaluation test");
    let input = "3+4+(5*6)";
    let mut stream = LexStream::init(input);

    let bytecode = gen_bytecode(&mut stream);
    let output = run_bytecode(&bytecode);
    println!("Calculated {}",output);
}