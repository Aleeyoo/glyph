//! miniLisp built-in functions: car, cdr, cons, eq, atom, +, -, *, /, print.
//!
//! Each builtin is a function pointer `fn(&[LispVal]) -> Result<LispVal, String>`.

use super::engine::LispVal;

pub fn builtin_car(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() != 1 { return Err("car: need 1 argument".into()); }
    match &args[0] {
        LispVal::Cons(car, _) => Ok(*car.clone()),
        _ => Err("car: need a pair".into()),
    }
}

pub fn builtin_cdr(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() != 1 { return Err("cdr: need 1 argument".into()); }
    match &args[0] {
        LispVal::Cons(_, cdr) => Ok(*cdr.clone()),
        _ => Err("cdr: need a pair".into()),
    }
}

pub fn builtin_cons(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() != 2 { return Err("cons: need 2 arguments".into()); }
    Ok(LispVal::Cons(Box::new(args[0].clone()), Box::new(args[1].clone())))
}

pub fn builtin_eq(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() != 2 { return Err("eq: need 2 arguments".into()); }
    let eq = match (&args[0], &args[1]) {
        (LispVal::Integer(a), LispVal::Integer(b)) => a == b,
        (LispVal::Symbol(a), LispVal::Symbol(b)) => a == b,
        (LispVal::String(a), LispVal::String(b)) => a == b,
        (LispVal::Nil, LispVal::Nil) => true,
        _ => false,
    };
    if eq { Ok(LispVal::Symbol("t".into())) } else { Ok(LispVal::Nil) }
}

pub fn builtin_atom(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() != 1 { return Err("atom: need 1 argument".into()); }
    match &args[0] {
        LispVal::Cons(..) => Ok(LispVal::Nil),
        _ => Ok(LispVal::Symbol("t".into())),
    }
}

pub fn builtin_add(args: &[LispVal]) -> Result<LispVal, String> {
    let mut sum: i64 = 0;
    for a in args {
        if let LispVal::Integer(n) = a { sum += n; }
        else { return Err("+: expected integer".into()); }
    }
    Ok(LispVal::Integer(sum))
}

pub fn builtin_sub(args: &[LispVal]) -> Result<LispVal, String> {
    if args.is_empty() { return Err("-: need at least 1 argument".into()); }
    if args.len() == 1 {
        if let LispVal::Integer(n) = &args[0] { return Ok(LispVal::Integer(-n)); }
    }
    let mut result = if let LispVal::Integer(n) = &args[0] { *n } else { return Err("-: expected integer".into()); };
    for a in &args[1..] {
        if let LispVal::Integer(n) = a { result -= n; }
        else { return Err("-: expected integer".into()); }
    }
    Ok(LispVal::Integer(result))
}

pub fn builtin_mul(args: &[LispVal]) -> Result<LispVal, String> {
    let mut product: i64 = 1;
    for a in args {
        if let LispVal::Integer(n) = a { product *= n; }
        else { return Err("*: expected integer".into()); }
    }
    Ok(LispVal::Integer(product))
}

pub fn builtin_div(args: &[LispVal]) -> Result<LispVal, String> {
    if args.len() < 2 { return Err("/: need at least 2 arguments".into()); }
    let mut result = if let LispVal::Integer(n) = &args[0] { *n } else { return Err("/: expected integer".into()); };
    for a in &args[1..] {
        if let LispVal::Integer(n) = a {
            if *n == 0 { return Err("/: division by zero".into()); }
            result /= n;
        }
        else { return Err("/: expected integer".into()); }
    }
    Ok(LispVal::Integer(result))
}

pub fn builtin_print(args: &[LispVal]) -> Result<LispVal, String> {
    let s: Vec<String> = args.iter().map(|v| format!("{}", v)).collect();
    println!("{}", s.join(" "));
    Ok(LispVal::Nil)
}
