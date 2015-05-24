use lexer::Token;
use lexer;
use expr::Expr;

use std::rc::Rc;
use std::fs::File;
use std::io::Read;

fn read_quote<'a> (xs:&'a [Token])->(Expr, &'a [Token]) {
    if xs.len() == 0 {
        error! ("Error parsing quote: not enough arguments");
        (Expr::Nil, &[])
    } else {
        let (e,r) = read_expr(&xs[0], &xs[1..]);
        (Expr::Quote(Rc::new(e)), r)
    }
}

fn read_unquote<'a> (xs:&'a [Token])->(Expr, &'a [Token]) {
    if xs.len() == 0 {
        error! ("Error parsing quote: not enough arguments");
        (Expr::Nil, &[])
    } else {
        let (e,r) = read_expr(&xs[0], &xs[1..]);
        (Expr::Unquote(Rc::new(e)), r)
    }
}

fn read_quasiquote<'a> (xs:&'a [Token])->(Expr, &'a [Token]) {
    if xs.len() == 0 {
        error! ("Error parsing quote: not enough arguments");
        (Expr::Nil, &[])
    } else {
        let (e,r) = read_expr(&xs[0], &xs[1..]);
        (Expr::Quasiquote(Rc::new(e)), r)
    }
}

fn read_paren<'a> (xs:&'a [Token])->(Expr,&'a[Token]) {
    if xs.len() == 0 {
        error! ("Error parsing '(: closing parenthesis not found");
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
        Token::Quasiquote => read_quasiquote(xs),
        Token::Unquote => read_unquote(xs),
        Token::OpeningParen => read_paren(xs),
        Token::ClosingParen => {
            error!("Parse error: closing parenthesis doesn't match opening one");
            (Expr::Nil, &[])
        }
    }
}

pub fn read(xs: &[Token])-> Vec<Rc<Expr>> {
    let mut res:Vec<Rc<Expr>> = vec!();
    let mut tokens = xs;
    if tokens.len() == 0 {
        res
    } else {
        loop {
            let (e,r) = read_expr (&tokens[0], &tokens[1..]);
            res.push(Rc::new(e));
            if r.len() == 0 {
                return res;
            } else {
                tokens = r;
            }
        }
    }
}

pub fn read_str(s:&str) -> Vec<Rc<Expr>> {
    let o = lexer::tokenize(s);
    match o {
        None => {
            error!("Lexer failed");
            vec!()
        }
        Some(v) => read(&*v)
    }
}

pub fn read_file(s:&str) -> Vec<Rc<Expr>> {
    let res:Vec<Rc<Expr>> = vec!();
    let r = File::open(s);
    let mut file:&File;

    match r {
        Err(_) => {
            error!("Error opening file {}", s);
            return res;
        },
        Ok(ref f) => {
            file = f;
        }
    }
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    read_str (content.as_ref())
}
