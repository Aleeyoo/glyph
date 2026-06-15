//! miniLisp tokenizer + parser (S-expression reader).
//!
//! Minimal Lisp dialect for editor configuration and scripting.
//! Parses S-expressions into a simple AST.

use std::fmt;
use std::collections::HashMap;

/// miniLisp value types.
#[derive(Debug, Clone, PartialEq)]
pub enum LispVal {
    Nil,
    Integer(i64),
    String(String),
    Symbol(String),
    Cons(Box<LispVal>, Box<LispVal>),
    Func(fn(&[LispVal]) -> Result<LispVal, String>),
    /// User-defined closure: captured env, parameter names, body expression.
    Closure(Box<Env>, Vec<String>, Box<LispVal>),
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
            LispVal::Closure(_, _, _) => write!(f, "#<closure>"),
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

/// Evaluation environment: a chain of scopes.
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub bindings: HashMap<String, LispVal>,
    pub outer: Option<Box<Env>>,
}

impl Env {
    /// Create a new top-level environment.
    pub fn new() -> Self {
        Env {
            bindings: HashMap::new(),
            outer: None,
        }
    }

    /// Create an environment with the given bindings (and optional outer env).
    pub fn with_outer(outer: Box<Env>) -> Self {
        Env {
            bindings: HashMap::new(),
            outer: Some(outer),
        }
    }

    /// Look up a symbol in this environment, searching parent scopes.
    pub fn get(&self, name: &str) -> Option<LispVal> {
        match self.bindings.get(name) {
            Some(val) => Some(val.clone()),
            None => self.outer.as_ref().and_then(|o| o.get(name)),
        }
    }

    /// Define a binding in this environment.
    pub fn define(&mut self, name: String, val: LispVal) {
        self.bindings.insert(name, val);
    }
}

/// Populate an environment with built-in functions.
pub fn populate_builtins(env: &mut Env) {
    use crate::extend::builtins::*;
    let mut builtins: Vec<(&str, fn(&[LispVal]) -> Result<LispVal, String>)> = vec![
        ("car", builtin_car),
        ("cdr", builtin_cdr),
        ("cons", builtin_cons),
        ("eq", builtin_eq),
        ("atom", builtin_atom),
        ("+", builtin_add),
        ("-", builtin_sub),
        ("*", builtin_mul),
        ("/", builtin_div),
        ("print", builtin_print),
    ];
    // Register editor-call
    builtins.push(("editor-call", crate::extend::integration::editor_call));
    for (name, func) in builtins {
        env.define(name.to_string(), LispVal::Func(func));
    }
    env.define("t".to_string(), LispVal::Symbol("t".to_string()));
}

/// Format a Lisp error message, truncating to 80 characters for display.
pub fn format_lisp_error(err: &str) -> String {
    if err.len() <= 80 {
        err.to_string()
    } else {
        format!("{}...", &err[..77])
    }
}

/// Safe wrapper around `eval` that returns `LispVal::Nil` on error.
pub fn safe_eval(env: &mut Env, expr: &LispVal) -> LispVal {
    match eval(env, expr) {
        Ok(val) => val,
        Err(_) => LispVal::Nil,
    }
}

/// Evaluate an expression in the given environment.
pub fn eval(env: &mut Env, expr: &LispVal) -> Result<LispVal, String> {
    match expr {
        LispVal::Symbol(s) => {
            env.get(s).ok_or_else(|| format!("undefined symbol: {}", s))
        }
        LispVal::Integer(_) | LispVal::String(_) | LispVal::Nil | LispVal::Func(_) | LispVal::Closure(..) => {
            Ok(expr.clone())
        }
        LispVal::Cons(car, cdr) => {
            // Check for special forms BEFORE evaluating car
            if let LispVal::Symbol(s) = car.as_ref() {
                match s.as_str() {
                    "quote" => {
                        // (quote expr) – return the single argument unevaluated
                        return list_first(cdr).ok_or_else(|| "quote: need an argument".into()).map(|v| v.clone());
                    }
                    "if" => {
                        // (if test then else?)
                        let items = list_to_vec(cdr);
                        if items.len() < 2 {
                            return Err("if: need at least 2 arguments (test then)".into());
                        }
                        let test = eval(env, &items[0])?;
                        let cond_true = match &test {
                            LispVal::Nil => false,
                            _ => true,
                        };
                        if cond_true {
                            return eval(env, &items[1]);
                        } else if items.len() >= 3 {
                            return eval(env, &items[2]);
                        } else {
                            return Ok(LispVal::Nil);
                        }
                    }
                    "cond" => {
                        // (cond (test expr) ... (else expr))
                        let clauses = list_to_vec(cdr);
                        for clause in &clauses {
                            let pair = list_to_vec(clause);
                            if pair.len() != 2 {
                                return Err("cond: each clause must be (test expr)".into());
                            }
                            // Special case: (else expr) should match without evaluating "else"
                            if let LispVal::Symbol(sym) = &pair[0] {
                                if sym == "else" {
                                    return eval(env, &pair[1]);
                                }
                            }
                            let test = eval(env, &pair[0])?;
                            if !matches!(test, LispVal::Nil) {
                                return eval(env, &pair[1]);
                            }
                        }
                        return Ok(LispVal::Nil);
                    }
                    "define" => {
                        // (define name value) or (define (fn args) body)
                        let items = list_to_vec(cdr);
                        if items.is_empty() {
                            return Err("define: need at least 1 argument".into());
                        }
                        // Check if it's function definition: (define (fn args) body)
                        if let LispVal::Cons(fn_head, fn_tail) = &items[0] {
                            let fn_name = match fn_head.as_ref() {
                                LispVal::Symbol(s) => s.clone(),
                                _ => return Err("define: invalid function name".into()),
                            };
                            let params = list_to_vec(fn_tail);
                            let param_names: Vec<String> = params.iter().map(|p| {
                                match p {
                                    LispVal::Symbol(s) => s.clone(),
                                    _ => panic!("define: non-symbol parameter"),
                                }
                            }).collect();
                            if items.len() < 2 {
                                return Err("define: need a body for function".into());
                            }
                            let body = items[1].clone();
                            let closure = LispVal::Closure(Box::new(env.clone()), param_names, Box::new(body));
                            env.define(fn_name, closure);
                            return Ok(LispVal::Symbol("ok".to_string()));
                        } else {
                            let name = match &items[0] {
                                LispVal::Symbol(s) => s.clone(),
                                _ => return Err("define: first argument must be a symbol".into()),
                            };
                            if items.len() < 2 {
                                return Err("define: need a value".into());
                            }
                            let val = eval(env, &items[1])?;
                            env.define(name, val);
                            return Ok(LispVal::Symbol("ok".to_string()));
                        }
                    }
                    "lambda" => {
                        // (lambda (params) body)
                        let items = list_to_vec(cdr);
                        if items.len() < 2 {
                            return Err("lambda: need (params) body".into());
                        }
                        let params = match &items[0] {
                            LispVal::Nil => Vec::new(),
                            cons @ LispVal::Cons(..) => list_to_vec(cons),
                            _ => return Err("lambda: params must be a list".into()),
                        };
                        let param_names: Vec<String> = params.iter().map(|p| {
                            match p {
                                LispVal::Symbol(s) => Ok::<String, String>(s.clone()),
                                _ => Err::<String, String>("lambda: each param must be a symbol".into()),
                            }
                        }).collect::<Result<Vec<_>, _>>()?;
                        let body = items[1].clone();
                        return Ok(LispVal::Closure(Box::new(env.clone()), param_names, Box::new(body)));
                    }
                    _ => {}
                }
            }

            // Not a special form — evaluate car and apply
            let car_evaled = eval(env, car)?;
            let args = eval_args(env, cdr)?;
            match car_evaled {
                LispVal::Func(func) => func(&args),
                LispVal::Closure(captured_env, params, body) => {
                    if args.len() != params.len() {
                        return Err(format!(
                            "lambda: expected {} arguments, got {}",
                            params.len(),
                            args.len()
                        ));
                    }
                    let mut local_env = Env::with_outer(captured_env.clone());
                    for (param, arg) in params.iter().zip(args.into_iter()) {
                        local_env.define(param.clone(), arg);
                    }
                    eval(&mut local_env, &body)
                }
                _ => Err(format!("cannot call non-function: {}", car_evaled)),
            }
        }
    }
}

/// Evaluate each element in a cons-chain list.
fn eval_args(env: &mut Env, args: &LispVal) -> Result<Vec<LispVal>, String> {
    let mut result = Vec::new();
    let mut current = args;
    while let LispVal::Cons(car, cdr) = current {
        result.push(eval(env, car)?);
        current = cdr;
    }
    Ok(result)
}

/// Convert a cons-chain to a Vec of references.
fn list_to_vec(list: &LispVal) -> Vec<&LispVal> {
    let mut result = Vec::new();
    let mut current = list;
    while let LispVal::Cons(car, cdr) = current {
        result.push(car.as_ref());
        current = cdr;
    }
    result
}

/// Get the first element of a list, or None if it's nil.
fn list_first(list: &LispVal) -> Option<&LispVal> {
    match list {
        LispVal::Cons(car, _) => Some(car.as_ref()),
        LispVal::Nil => None,
        _ => None,
    }
}
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

    // -----------------------------------------------------------------------
    // eval tests
    // -----------------------------------------------------------------------

    /// Helper: parse + eval in the default environment.
    fn eval_str(input: &str) -> Result<LispVal, String> {
        let expr = read(input)?;
        let mut env = Env::new();
        populate_builtins(&mut env);
        eval(&mut env, &expr)
    }

    /// Helper: define a variable then evaluate subsequent expressions.
    fn eval_with_defs(defs: &[&str], expr: &str) -> Result<LispVal, String> {
        let mut env = Env::new();
        populate_builtins(&mut env);
        for def in defs {
            let d = read(def)?;
            eval(&mut env, &d)?;
        }
        let e = read(expr)?;
        eval(&mut env, &e)
    }

    #[test]
    fn eval_add() {
        let result = eval_str("(+ 1 2)").unwrap();
        assert_eq!(format!("{}", result), "3");
    }

    #[test]
    fn eval_car_quote() {
        let result = eval_str("(car (quote (1 2)))").unwrap();
        assert_eq!(format!("{}", result), "1");
    }

    #[test]
    fn eval_define_then_lookup() {
        let result = eval_with_defs(&["(define x 5)"], "x").unwrap();
        assert_eq!(format!("{}", result), "5");
    }

    #[test]
    fn eval_if_true() {
        let result = eval_str("(if t 1 2)").unwrap();
        assert_eq!(format!("{}", result), "1");
    }

    #[test]
    fn eval_lambda_application() {
        let result = eval_str("((lambda (x) (+ x 1)) 2)").unwrap();
        assert_eq!(format!("{}", result), "3");
    }

    #[test]
    fn eval_cdr_quote() {
        let result = eval_str("(cdr (quote (1 2 3)))").unwrap();
        assert_eq!(format!("{}", result), "(2 3)");
    }

    #[test]
    fn eval_cons() {
        let result = eval_str("(cons 1 (quote (2 3)))").unwrap();
        assert_eq!(format!("{}", result), "(1 2 3)");
    }

    #[test]
    fn eval_eq_true() {
        let result = eval_str("(eq 42 42)").unwrap();
        assert_eq!(format!("{}", result), "t");
    }

    #[test]
    fn eval_define_function() {
        let result = eval_with_defs(&["(define (double x) (* x 2))"], "(double 5)").unwrap();
        assert_eq!(format!("{}", result), "10");
    }

    #[test]
    fn eval_cond_first_clause() {
        let result = eval_str("(cond ((eq 1 2) 10) (t 20))").unwrap();
        assert_eq!(format!("{}", result), "20");
    }

    #[test]
    fn eval_nested_lambda() {
        let result = eval_str("(((lambda (x) (lambda (y) (+ x y))) 3) 4)").unwrap();
        assert_eq!(format!("{}", result), "7");
    }

    #[test]
    fn eval_error_returns_err_not_panic() {
        // car on an integer is invalid – should return Err, not panic
        let result = eval_str("(car 1)");
        assert!(result.is_err(), "expected Err, got Ok: {:?}", result);

        // undefined symbol
        let result = eval_str("nonesuch");
        assert!(result.is_err(), "expected Err for undefined symbol");

        // malformed if
        let result = eval_str("(if)");
        assert!(result.is_err(), "expected Err for malformed if");

        // calling a non-function
        let result = eval_str("(1 2 3)");
        assert!(result.is_err(), "expected Err for calling non-function");
    }

    #[test]
    fn safe_eval_returns_nil_on_error() {
        let mut env = Env::new();
        populate_builtins(&mut env);
        let expr = read("(car 1)").unwrap();
        let result = safe_eval(&mut env, &expr);
        assert_eq!(result, LispVal::Nil, "safe_eval should return Nil on error");
    }

    #[test]
    fn format_lisp_error_truncates_long() {
        let long = "a".repeat(200);
        let formatted = format_lisp_error(&long);
        assert!(formatted.len() <= 80, "formatted error should be <= 80 chars");
        assert!(formatted.ends_with("..."), "truncated error should end with ...");
    }

    #[test]
    fn format_lisp_error_keeps_short() {
        let short = "undefined symbol: foo";
        let formatted = format_lisp_error(short);
        assert_eq!(formatted, short, "short error should not be altered");
    }
}
