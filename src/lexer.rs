use crate::tokens::{keyword_or_ident, Token, TokenKind};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            input: src.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.bump();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        // '//' bis Zeilenende
        if self.peek() == Some('/') && self.peek_next() == Some('/') {
            while let Some(c) = self.peek() {
                self.bump();
                if c == '\n' {
                    break;
                }
            }
        }
    }

    fn read_number(&mut self, start_col: usize) -> Token {
        let line = self.line;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        let val: i64 = s.parse().unwrap();
        Token {
            kind: TokenKind::Int(val),
            line,
            col: start_col,
        }
    }

    fn read_ident(&mut self, start_col: usize) -> Token {
        let line = self.line;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(c);
                self.bump();
            } else {
                break;
            }
        }
        Token {
            kind: keyword_or_ident(&s),
            line,
            col: start_col,
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            // whitespace & comments überspringen
            self.skip_whitespace();
            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                self.skip_comment();
                continue;
            }
            break;
        }

        let start_col = self.col;
        let start_line = self.line;

        match self.peek() {
            None => {
                return Token {
                    kind: TokenKind::Eof,
                    line: start_line,
                    col: start_col,
                }
            }

            Some(c) if c.is_ascii_digit() => return self.read_number(start_col),
            Some(c) if c.is_ascii_alphabetic() || c == '_' => return self.read_ident(start_col),

            Some('=') => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    return Token {
                        kind: TokenKind::EqEq,
                        line: start_line,
                        col: start_col,
                    };
                }
                return Token {
                    kind: TokenKind::Assign,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('!') => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    return Token {
                        kind: TokenKind::NotEq,
                        line: start_line,
                        col: start_col,
                    };
                }
                return Token {
                    kind: TokenKind::NotEq,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('<') => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    return Token {
                        kind: TokenKind::Lte,
                        line: start_line,
                        col: start_col,
                    };
                }
                return Token {
                    kind: TokenKind::Lt,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('>') => {
                self.bump();
                if self.peek() == Some('=') {
                    self.bump();
                    return Token {
                        kind: TokenKind::Gte,
                        line: start_line,
                        col: start_col,
                    };
                }
                return Token {
                    kind: TokenKind::Gt,
                    line: start_line,
                    col: start_col,
                };
            }

            Some('+') => {
                self.bump();
                return Token {
                    kind: TokenKind::Plus,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('-') => {
                self.bump();
                return Token {
                    kind: TokenKind::Minus,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('*') => {
                self.bump();
                return Token {
                    kind: TokenKind::Star,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('/') => {
                self.bump();
                return Token {
                    kind: TokenKind::Slash,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('%') => {
                self.bump();
                return Token {
                    kind: TokenKind::Percent,
                    line: start_line,
                    col: start_col,
                };
            }
            Some('(') => {
                self.bump();
                return Token {
                    kind: TokenKind::LParen,
                    line: start_line,
                    col: start_col,
                };
            }
            Some(')') => {
                self.bump();
                return Token {
                    kind: TokenKind::RParen,
                    line: start_line,
                    col: start_col,
                };
            }
            Some(';') => {
                self.bump();
                return Token {
                    kind: TokenKind::Semicolon,
                    line: start_line,
                    col: start_col,
                };
            }
            Some(_) => {
                self.bump();
                return Token {
                    kind: TokenKind::Unknown,
                    line: start_line,
                    col: start_col,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenKind::*;

    fn kinds(src: &str) -> Vec<TokenKind> {
        let mut lx = Lexer::new(src);
        let mut out = Vec::new();
        loop {
            let t = lx.next_token();
            if t.kind == Eof {
                out.push(Eof);
                break;
            }
            out.push(t.kind);
        }
        out
    }
    #[test]
    fn lex_set_bind_expr() {
        let got = kinds("set x = 1 + 2 * (3 + 4);");
        let expect = vec![
            Set,
            Ident("x".into()),
            Assign,
            Int(1),
            Plus,
            Int(2),
            Star,
            LParen,
            Int(3),
            Plus,
            Int(4),
            RParen,
            Semicolon,
            Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn lex_compare_ops() {
        let got = kinds("1 <= 2 == 2 != 3 >= 0");
        let expect = vec![
            Int(1),
            Lte,
            Int(2),
            EqEq,
            Int(2),
            NotEq,
            Int(3),
            Gte,
            Int(0),
            Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn lex_comments_and_ws() {
        let got = kinds("// comment\nset a=5 // hi\n a = a + 1");
        let expect = vec![
            Set,
            Ident("a".into()),
            Assign,
            Int(5),
            Ident("a".into()),
            Assign,
            Ident("a".into()),
            Plus,
            Int(1),
            Eof,
        ];
        assert_eq!(got, expect);
    }
}
