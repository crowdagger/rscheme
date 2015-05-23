mod lexer;
mod list;
mod read;
mod eval;

use std::rc::Rc;

fn main() {
    // let l:list::List<u32> = list::List::new().cons(2).cons(1);
    // let l = l.map(|x| 2 * x);
    // println! ("{}", l.count());
    // println! ("{:?}", l.car());
    // println! ("{:?}", l.cdr().car());
    // println! ("{:?}", l.cdr().cdr().car());

    // let mut h = context::new ();
    // h.insert ("toto".to_string(), Rc::new(read::Expr::Nil));
    // let mut h2 = h.clone ();
    // h.insert ("titi".to_string(), Rc::new(read::Expr::Nil));
    // h2.insert ("toto".to_string(), Rc::new(read::Expr::Nil));
    // println! ("{:?}", h);
    // println! ("{:?}", h2);

    let h = eval::Context::new(read::Expr::Nil);
    let h2 = h.add_env("toto".to_string(), Rc::new(read::Expr::Nil));
    h2.lookup(&"toto".to_string());
    println!("{:?}", h2);


    
//    let s:&str = "(1 2 (3.0 \"toto\" ) 4)";
    //    let s = "(1 2)";
    //    let s = "(if titi 2 3)";
    //    let s = "(def x (if titi (- titi (+ 2 (* 4 (/ 2.5 5)))) (+ 3 5 6)))";
    //    let s = "(cons 1 (cons (- 2 1.0) ()))";
    let s = "(def f (lambda (x) (* 2 x)))";
    let e = read::read_str(s);
    println!("{:?}", &e);
    let c = eval::Context::new(e);
    let c = c.add_env("titi".to_string(),Rc::new(read::Expr::Integer(42)));
    //let c = c.add_env("titi".to_string(),Rc::new(read::Expr::Nil));
    println!("Before eval:\n {:?}", &c);
    let mut c = c.eval();
    println!("After eval:\n{:?}", &c);
    let s = "(f 42)";
    c = c.set_expr (read::read_str(s));
    println!("Before eval:\n {:?}", &c);
    println!("After:\n{:?}",c.eval());
}

