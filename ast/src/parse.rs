use crate::Expression;

use thiserror::Error;

use std::str::Chars;
use std::mem;

macro_rules! T {
    ($k:ident) => {
        TokenKind::$k
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Let,
    In,

    BSlash,
    Eq,
    Dot,
    LParen,
    RParen,

    Id,

    True,
    False,

    EOF,
    Error,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            T![Let]    => write!(f, r"let"),
            T![In]     => write!(f, r"in"),
            T![BSlash] => write!(f, r"\"),
            T![Eq]     => write!(f, "="),
            T![Dot]    => write!(f, "."),
            T![LParen] => write!(f, "("),
            T![RParen] => write!(f, ")"),
            T![Id]     => write!(f, "ID"),
            T![True]   => write!(f, "true"),
            T![False]  => write!(f, "false"),
            T![EOF]    => write!(f, "End of File"),
            T![Error]  => write!(f, "Error"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    pos: usize,
    len: usize,
}

impl Token {
    fn new(kind: TokenKind, pos: usize, len: usize) -> Token {
        Token { kind, pos, len }
    }
}

impl Default for Token {
    fn default() -> Token {
        Token::new(T![EOF], 0, 0)
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseError {
    #[error("Unexpected token '{unexpected}' found at position {pos}, expecting {}.",
        expected.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(", ")
    )]
    UnexpectedToken {
        unexpected: TokenKind,
        expected: Vec<TokenKind>,
        pos: usize,
    },
    #[error("Invalid Token '{lexeme}' at position {pos}.")]
    InvalidToken {
        lexeme: String,
        pos: usize,
    },
}

struct ParseContext<'src> {
    src: &'src str,
    pos: usize,
    chr: Chars<'src>,
    buf: [Token; 4],
}

impl ParseContext<'_> {
    fn new(src: &str) -> ParseContext<'_> {
        let pos = 0;
        let chr = src.chars();
        let buf = [Token::default(); 4];
        let mut ctx = ParseContext { src, pos, chr, buf };

        for _ in 0..buf.len() { ctx.next(); }
        ctx
    }

    fn expect(&mut self, k: TokenKind) -> Result<Token, ParseError> {
        let tok = self.peek_nth(0);
        if tok.kind != k { Err(ParseError::UnexpectedToken {
            unexpected: tok.kind,
            expected: vec![k],
            pos: tok.pos,
        })}
        else { Ok(self.next()) }
    }

    fn next(&mut self) -> Token {
        let mut tok = self.lex();
        mem::swap(&mut tok, &mut self.buf[0]);
        self.buf.rotate_left(1);
        tok
    }

    fn peek_nth(&self, n: usize) -> Token {
        if n > self.buf.len() {
            panic!("Parser cannot peek {n} tokens ahead with a buffer size of {}.", self.buf.len())
        }
        self.buf[n]
    }

    fn lexeme(&self, tok: Token) -> &str {
        &self.src[tok.pos..(tok.pos+tok.len)]
    }

    fn lex(&mut self) -> Token {
        'main: loop {
            let pos = self.pos;
            let Some(c) = self.next_char() else {
                return Token::new(T![EOF], pos, 1);
            };
            
            let kind = match c {
                '\\' => T![BSlash],
                '='  => T![Eq],
                '.'  => T![Dot],
                '('  => T![LParen],
                ')'  => T![RParen],

                c if c.is_alphabetic() => {
                    let mut s = String::from(c);
                    let valid_char = |c: char| c.is_alphabetic() || c == '\'' || c == '_';
                    while let Some(c_) = self.peek_char() && valid_char(c_) {
                        s.push(c_);
                        self.next_char();
                    }

                    let kind = match s.as_str() {
                        "let"   => T![Let],
                        "in"    => T![In],
                        "true"  => T![True],
                        "false" => T![False],
                        _ => T![Id],
                    };
                    return Token::new(kind, pos, self.pos - pos);
                }

                c if c.is_whitespace() => loop {
                    let Some(c) = self.peek_char() else {
                        return Token::new(T![EOF], self.pos, 1);
                    };
                    if c.is_whitespace() { self.next_char(); }
                    else { continue 'main; }
                }

                _ => return Token::new(T![Error], pos, 1),
            };

            return Token::new(kind, pos, self.pos - pos);
        }
    }

    #[inline]
    fn next_char(&mut self) -> Option<char> {
        let c = self.chr.next();
        self.pos += if c.is_some() { 1 } else { 0 };
        c
    }

    #[inline]
    fn peek_char(&mut self) -> Option<char> {
        self.chr.clone().next()
    }

}

pub fn parse(src: &str) -> Result<Expression, ParseError> {
    let mut ctx = ParseContext::new(src);
    parse_expr(&mut ctx)
}

fn parse_expr(ctx: &mut ParseContext) -> Result<Expression, ParseError> {
    let tok = ctx.next();
    let mut lhs = match tok.kind {
        T![LParen] => {
            let e = parse_expr(ctx)?;
            ctx.expect(T![RParen])?;
            e
        }

        T![Id] => {
            let name = ctx.lexeme(tok).into();
            Expression::Var { name }
        }

        T![BSlash] => {
            let name = ctx.expect(T![Id])?;
            let name = ctx.lexeme(name).into();
            ctx.expect(T![Dot])?;
            let e = parse_expr(ctx)?.into();
            Expression::Abs { name, e }
        }

        T![Let] => {
            let name = ctx.expect(T![Id])?;
            let name = ctx.lexeme(name).into();
            ctx.expect(T![Eq])?;
            let e0 = parse_expr(ctx)?.into();
            ctx.expect(T![In])?;
            let e1 = parse_expr(ctx)?.into();
            Expression::Let { name, e0, e1 }
        }

        T![True]  => Expression::True,
        T![False] => Expression::False,

        T![Error] => return Err(ParseError::InvalidToken {
            lexeme: ctx.lexeme(tok).to_string(),
            pos: tok.pos,
        }),

        _ => return Err(ParseError::UnexpectedToken {
            unexpected: tok.kind,
            expected: vec![T![LParen], T![Id], T![BSlash], T![Let], T![True], T![False]],
            pos: tok.pos
        })
    };

    loop {
        let tok = ctx.peek_nth(0);
        if matches!(tok.kind, T![EOF] | T![RParen] | T![In]) { break; }
        if tok.kind == T![Error] {
            return Err(ParseError::InvalidToken {
                lexeme: ctx.lexeme(tok).to_string(),
                pos: tok.pos,
            })
        }
        let rhs = parse_expr(ctx)?;
        lhs = Expression::App { e0: lhs.into(), e1: rhs.into() };
    }
    
    Ok(lhs)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_lexing() {
        let mut ctx = ParseContext::new(r"let in \=.() hello true false -");
        assert_eq!(ctx.next(), Token::new(T![Let], 0, 3));
        assert_eq!(ctx.next(), Token::new(T![In] , 4, 2));
        assert_eq!(ctx.next(), Token::new(T![BSlash], 7, 1));
        assert_eq!(ctx.next(), Token::new(T![Eq], 8, 1));
        assert_eq!(ctx.next(), Token::new(T![Dot], 9, 1));
        assert_eq!(ctx.next(), Token::new(T![LParen], 10, 1));
        assert_eq!(ctx.next(), Token::new(T![RParen], 11, 1));
        assert_eq!(ctx.next(), Token::new(T![Id], 13, 5));
        assert_eq!(ctx.next(), Token::new(T![True], 19, 4));
        assert_eq!(ctx.next(), Token::new(T![False], 24, 5));
        assert_eq!(ctx.next(), Token::new(T![Error], 30, 1));
        assert_eq!(ctx.next(), Token::new(T![EOF], 31, 1));
    }

    #[test]
    fn test_abs() {
        let e_parse = parse(r"\x. x");
        let e_correct = Expression::Abs {
            name: "x".into(),
            e: Expression::Var { name: "x".into() }.into(),
        };

        assert_eq!(e_parse, Ok(e_correct))
    }

    #[test]
    fn test_app() {
        let e_parse = parse(r"(\x. x) true");
        let e_correct = Expression::App {
            e0: Expression::Abs {
                name: "x".into(),
                e: Expression::Var { name: "x".into() }.into(),
            }.into(),
            e1: Expression::True.into(),
        };

        assert_eq!(e_parse, Ok(e_correct))
    }

    #[test]
    fn test_let() {
        let e_parse = parse(r"let id = (\x. x) in id true");
        let e_correct = Expression::Let {
            name: "id".into(),
            e0: Expression::Abs {
                name: "x".into(),
                e: Expression::Var { name: "x".into() }.into(),
            }.into(),
            e1: Expression::App {
                e0: Expression::Var { name: "id".into() }.into(),
                e1: Expression::True.into(),
            }.into(),
        };

            
        assert_eq!(e_parse, Ok(e_correct))
    }
}
