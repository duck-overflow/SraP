use srap::lexer::Lexer;
use srap::tokens::TokenKind;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut lx = Lexer::new(&buf);
    loop {
        let t = lx.next_token();
        println!("{:?}", t);
        if t.kind == TokenKind::Eof {
            break;
        }
    }
}
