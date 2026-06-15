//! miniLisp tokenizer + parser (S-expression reader).
//!
//! Minimal Lisp dialect for editor configuration and scripting.
//! Parses S-expressions into a simple AST.

use std::fmt;

/// miniLisp value types.
#[derive(Debug, Clone)]
pub enum LispVal {
    Nil,
    Integer(i64),
    String(String),
    Symbol(String),
    Cons(Box<LispVal>, Box<LispVal>),
    Func(fn(&[LispVal]) -> Result<LispVal, String>),
}

impl fmt::Display for LispVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LispVal::Nil => write!(f, "nil"),
            LispVal::Integer(n) => write!(f, "{}", n),
            LispVal::String(s) => write!(f, "\"{}\"", s),
            LispVal::Symbol(s) => write!(f, "{}", s),
            LispVal::Cons(car, cdr) => {
                write!(f, "({}", car)?;
                let mut tail = cdr;
                while let LispVal::Cons(c, d) = tail.as_ref() {
                    write!(f, " {}", c)?;
                    tail = d;
                }
                if let LispVal::Nil = tail.as_ref() {
                    write!(f, ")")
                } else {
                    write!(f, " . {})", tail)
                }
            }
            LispVal::Func(_) => write!(f, "#<function>"),
        }
    }
}

/// Token types for the S-expression lexer.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Dot,
    Quote,
    Number(i64),
    String(String),
    Symbol(String),
}

/// Lexer: string → tokens.
fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            '\'' => { tokens.push(Token::Quote); chars.next(); }
            '.' => {
                chars.next();
                if chars.peek().map(|c| c.is_whitespace() || *c == ')').unwrap_or(true) {
                    tokens.push(Token::Dot);
                } else {
                    return Err("Unexpected '.'".into());
                }
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' { chars.next(); break; }
                    s.push(c); chars.next();
                }
                tokens.push(Token::String(s));
            }
            ';' => { while chars.next().is_some() && chars.peek() != Some(&'\n') {} }
            c if c.is_ascii_whitespace() => { chars.next(); }
            '-' | '0'..='9' => {
                let mut num = String::new();
                if c == '-' { num.push('-'); chars.next(); }
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() { num.push(c); chars.next(); }
                    else { break; }
                }
                let n: i64 = num.parse().map_err(|_| format!("Bad number: {}", num))?;
                tokens.push(Token::Number(n));
            }
            _ => {
                let mut sym = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_whitespace() || "()\"';".contains(c) { break; }
                    sym.push(c); chars.next();
                }
                tokens.push(Token::Symbol(sym));
            }
        }
    }
    Ok(tokens)
}

/// Parser: tokens → LispVal.
fn parse(tokens: &[Token], pos: &mut usize) -> Result<Option<LispVal>, String> {
    if *pos >= tokens.len() { return Ok(None); }
    let tok = &tokens[*pos];
    *pos += 1;
    match tok {
        Token::LParen => {
            let mut items = Vec::new();
            while *pos < tokens.len() {
                if tokens[*pos] == Token::RParen { *pos += 1; break; }
                if tokens[*pos] == Token::Dot {
                    *pos += 1;
                    let rest = parse(tokens, pos)?.unwrap_or(LispVal::Nil);
                    if *pos < tokens.len() && tokens[*pos] == Token::RParen {
                        *pos += 1;
                        return Ok(Some(rest));
                    }
                    return Err("Expected ) after dotted pair".into());
                }
                if let Some(val) = parse(tokens, pos)? {
                    items.push(val);
                }
            }
            // Build cons chain from items
            let mut result = LispVal::Nil;
            for item in items.into_iter().rev() {
                result = LispVal::Cons(Box::new(item), Box::new(result));
            }
            Ok(Some(result))
        }
        Token::RParen => Err("Unexpected )".into()),
        Token::Quote => {
            let inner = parse(tokens, pos)?.unwrap_or(LispVal::Nil);
            Ok(Some(LispVal::Cons(
                Box::new(LispVal::Symbol("quote".into())),
                Box::new(LispVal::Cons(Box::new(inner), Box::new(LispVal::Nil))),
            )))
        }
        Token::Number(n) => Ok(Some(LispVal::Integer(*n))),
        Token::String(s) => Ok(Some(LispVal::String(s.clone()))),
        Token::Symbol(s) => Ok(Some(LispVal::Symbol(s.clone()))),
        Token::Dot => Err("Unexpected dot".into()),
    }
}

/// Parse an S-expression string into a LispVal.
pub fn read(input: &str) -> Result<LispVal, String> {
    let tokens = lex(input)?;
    let mut pos = 0;
    let mut results = Vec::new();
    while pos < tokens.len() {
        if let Some(val) = parse(&tokens, &mut pos)? {
            results.push(val);
        }
    }
    // If multiple top-level forms, wrap in a begin-like list
    if results.len() == 1 {
        Ok(results.into_iter().next().unwrap())
    } else {
        let mut list = LispVal::Nil;
        for item in results.into_iter().rev() {
            list = LispVal::Cons(Box::new(item), Box::new(list));
        }
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        let v = read("42").unwrap();
        assert!(matches!(v, LispVal::Integer(42)));
    }

    #[test]
    fn parse_symbol() {
        let v = read("hello").unwrap();
        assert!(matches!(v, LispVal::Symbol(ref s) if s == "hello"));
    }

    #[test]
    fn parse_string() {
        let v = read(r#""hello""#).unwrap();
        assert!(matches!(v, LispVal::String(ref s) if s == "hello"));
    }

    #[test]
    fn parse_list() {
        let v = read("(+ 1 2)").unwrap();
        assert!(matches!(v, LispVal::Cons(_, _)));
    }

    #[test]
    fn parse_nested_list() {
        let v = read("(define (f x) (+ x 1))").unwrap();
        assert!(matches!(v, LispVal::Cons(_, _)));
    }

    #[test]
    fn parse_quoted() {
        let v = read("'foo").unwrap();
        let s = format!("{}", v);
        assert_eq!(s, "(quote foo)");
    }

    #[test]
    fn parse_nil() {
        let v = read("()").unwrap();
        assert!(matches!(v, LispVal::Nil));
    }

    #[test]
    fn comment_skipped() {
        let v = read(";comment\n42").unwrap();
        assert!(matches!(v, LispVal::Integer(42)));
    }
}
