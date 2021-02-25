use std::env;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(PartialEq, Debug)]
enum Token {
    Num(i32),
    Plus,
    Minus,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Num(i) => write!(f, "Num {}", i),
            Token::Plus => write!(f, "Plus"),
            Token::Minus => write!(f, "Minus"),
        }
    }
}

struct Lexer {
    program: Vec<char>,
    size: usize,
    pos: usize,
}

impl Lexer {
    fn new(program: &String) -> Lexer {
        Lexer {
            program: program.chars().collect(),
            size: program.len(),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while self.cur().is_some() {
            let c = self.cur().unwrap();
            if c == '+' {
                tokens.push(Token::Plus);
            } else if c == '-' {
                tokens.push(Token::Minus);
            } else if c.is_ascii_digit() {
                let mut val: i32 = self.cur().unwrap().to_digit(10).unwrap() as i32;
                while self.next().is_some() && self.next().unwrap().is_ascii_digit() {
                    self.incr();
                    val *= 10;
                    val += self.cur().unwrap().to_digit(10).unwrap() as i32;
                }
                tokens.push(Token::Num(val));
            } else {
                panic!("cannot parse input");
            }
            self.incr();
        }
        tokens
    }

    fn incr(&mut self) {
        self.pos += 1;
    }

    fn next(&self) -> Option<char> {
        if self.size - 1 <= self.pos {
            return None;
        } else {
            return Some(self.program[self.pos + 1]);
        }
    }

    fn cur(&self) -> Option<char> {
        if self.size <= self.pos {
            return None;
        } else {
            return Some(self.program[self.pos]);
        }
    }
}

fn parse(program: &String) -> Vec<Token> {
    let mut lexer = Lexer::new(program);
    let tokens = lexer.tokenize();
    tokens
}

fn output_asm(tokens: Vec<Token>) -> () {
    println!("main:");
    // preprocessing
    println!("addi  sp, sp, -16");
    println!("sw    s0, 12(sp)");
    println!("addi  s0, sp, 16");

    let mut itr = tokens.iter();
    let mut val: i32 = itr
        .next()
        .map(|token| match *token {
            Token::Num(i) => i,
            _ => panic!("first token must be number"),
        })
        .unwrap();
    println!("li    a5, {}", val);
    loop {
        let some_op = itr.next();
        if let Some(op) = some_op {
            match *op {
                Token::Plus => {
                    let num = itr.next().expect("input is not a formula");
                    if let Token::Num(i) = num {
                        println!("addi   a5, a5, {}", i);
                    } else {
                        panic!("input is not a formula");
                    }
                }
                Token::Minus => {
                    let num = itr.next().expect("input is not a formula");
                    if let Token::Num(i) = num {
                        println!("addi   a5, a5, -{}", i);
                        val -= i;
                    } else {
                        panic!("input is not a formula");
                    }
                }
                _ => break,
            }
        } else {
            break;
        }
    }
    // postprocessing
    println!("mv    a0, a5");
    println!("lw    s0, 12(sp)");
    println!("addi  sp, sp, 16");
    println!("jr    ra");
}

fn main() {
    // read arguments
    let args: Vec<String> = env::args().collect();

    // read program from file
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not found");
    let mut program = String::new();
    file.read_to_string(&mut program)
        .expect("cannot read file contents");

    // parse
    let tokens = parse(&program);

    // adder
    output_asm(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number_1() {
        let program = "0".to_string();
        assert_eq!(vec![Token::Num(0)], parse(&program));
    }
    #[test]
    fn parse_formula_1() {
        let program = "1+1".to_string();
        assert_eq!(
            vec![Token::Num(1), Token::Plus, Token::Num(1)],
            parse(&program)
        );
    }

    #[test]
    fn parse_formula_2() {
        let program = "1-1".to_string();
        assert_eq!(
            vec![Token::Num(1), Token::Minus, Token::Num(1)],
            parse(&program)
        );
    }

    #[test]
    fn parse_formula_3() {
        let program = "1-2+3".to_string();
        assert_eq!(
            vec![
                Token::Num(1),
                Token::Minus,
                Token::Num(2),
                Token::Plus,
                Token::Num(3)
            ],
            parse(&program)
        );
    }
}
