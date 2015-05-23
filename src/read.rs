use lexer::Token;
use lexer;
use list;

use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Nil,
    Lambda(Rc<Expr>, Rc<Expr>, HashMap<String,Rc<Expr>>),
    Macro(Rc<Expr>, Rc<Expr>),
    Integer(i64),
    Float(f64),
    Ident(String),
    String(String),
    Quote(Rc<Expr>),
    Cons(Rc<Expr>, Rc<Expr>)
}

fn read_quote<'a> (xs:&'a [Token])->(Expr, &'a [Token]) {
    if xs.len() == 0 {
        println! ("Error parsing quote: not enough arguments");
        (Expr::Nil, &[])
    } else {
        let (e,r) = read_expr(&xs[0], &xs[1..]);
        (Expr::Quote(Rc::new(e)), r)
    }
}

fn read_paren<'a> (xs:&'a [Token])->(Expr,&'a[Token]) {
    if xs.len() == 0 {
        println! ("Error parsing '(: closing parenthesis not found");
        (Expr::Nil,&[])
    } else {
        let x:&Token=&xs[0];
        let xs = &xs[1..];
        match *x {
            Token::ClosingParen => (Expr::Nil, xs),
            _ => {
                let (e1, r1) = read_expr (x, xs);
                let (e2, r2) = read_paren (r1);
                (Expr::Cons(Rc::new(e1),
                             Rc::new(e2)), r2)
            }
        }
    }
}

fn read_expr<'a> (x:&Token, xs:&'a [Token])->(Expr,&'a [Token]) {
    match *x {
        Token::Integer(x) => (Expr::Integer(x), xs),
        Token::Float(x) => (Expr::Float(x), xs),
        Token::Ident(ref x) => (Expr::Ident(x.clone ()), xs),
        Token::String(ref x) => (Expr::String(x.clone()), xs),
        Token::Quote => read_quote(xs),
        Token::OpeningParen => read_paren(xs),
        Token::ClosingParen => {
            println!("Parse error: closing parenthesis doesn't match opening one");
            (Expr::Nil, &[])
        }
    }
}

pub fn read(xs: &[Token])-> Expr {
    if xs.len() == 0 {
        Expr::Nil
    } else {
        let (e,r) = read_expr (&xs[0], &xs[1..]);
        e
    }
}

pub fn read_str(s:&str) -> Expr {
    let o = lexer::tokenize(s);
    match o {
        None => {
            println!("Lexer failed");
            Expr::Nil
        }
        Some(v) => read(&*v)
    }
}
    
