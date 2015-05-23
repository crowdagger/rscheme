use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;
use std::fmt::Display;
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

impl Display for Expr {
    fn fmt(&self, formatter:&mut Formatter) -> Result<(),Error> {
        match *self {
            Expr::Nil => formatter.write_str("()"),
            Expr::Lambda(_,_,_) => formatter.write_str("#Lambda"),
            Expr::Macro(_,_) => formatter.write_str("#Macro"),
            Expr::Integer(x) => x.fmt(formatter),
            Expr::Float(x) => x.fmt(formatter),
            Expr::Ident(ref s) => s.fmt(formatter),
            Expr::String(ref s) => formatter.write_fmt(format_args!("\"{}\"",
                                                                    s.clone())),
            Expr::Quote(ref e) => {
                try!(formatter.write_str("'"));
                e.fmt(formatter)
            },
            Expr::Cons(ref e1, ref e2) => {
                try!(formatter.write_str("("));
                try!(e1.fmt(formatter));
                try!(formatter.write_str(" . "));
                try!(e2.fmt(formatter));
                formatter.write_str(")")
            }
        }
    }
}
