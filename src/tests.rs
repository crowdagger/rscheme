use eval;
use read;
use expr::Expr;

use std::rc::Rc;

fn eval_str(s:&str) -> Rc<Expr> {
    let mut c = eval::Context::new();
    let es = read::read_str(s);
    for e in es {
        c = c.eval_expr(e.clone());
    }
    c.expr.clone()
}

#[test]
fn test_non_existing_file () {
    let mut c = eval::Context::new();
    c = c.eval_file("/some/non/existing/file/unless/we/are/really/unluck");
    match *c.expr {
        Expr::Nil => (),
        _ => panic!("Return should be nil")
    }
}

fn compare (expected:&Expr, got:&Expr) {
    if expected != got {
        panic!("Expected: {:?}, got: {:?}", expected, got);
        }
}

#[test]
fn test_integer () {
    let s = "2";
    let exp:Expr = Expr::Integer(2);
    let e = eval_str(s);
    compare (&exp, &e) 
}

#[test]
fn test_float () {
    let s = "3.0";
    let exp:Expr = Expr::Float(3.0);
    let e = eval_str(s);
    compare (&exp, &e) 
}

#[test]
fn test_nil () {
    let s = "()";
    let exp:Expr = Expr::Nil;
    let e = eval_str(s);
    compare (&exp, &e) 
}


#[test]
fn test_list () {
    let s = "'(1 2)";
    let exp:Expr = Expr::Cons (Rc::new(Expr::Integer(1)),
                               Rc::new(Expr::Cons(Rc::new(Expr::Integer(2)),
                                                  Rc::new(Expr::Nil))));
    
    let e = eval_str(s);
    compare (&exp, &e) 
}


#[test]
fn test_def () {
    let s = "(def x \"Oi\")
             x";
    let exp:Expr = Expr::String("Oi".to_string());
    let e = eval_str(s);
    compare (&exp, &e) 
}

#[test]
fn test_lambda () {
    let s = "(def f (lambda (x)
                            (_* 2 x)))
             (f 3)";
    let exp:Expr = Expr::Integer(6);
    let e = eval_str(s);
    compare (&exp, &e) 
}

#[test]
fn test_macro () {
    let s = "(defmacro defn (name args body)
                            `(def ,name
                                  (lambda ,args
                                          ,body)))
             (defn f (x) (_+ x 3.0))
             (f 2)";
    let exp:Expr = Expr::Float(5.0);
    let e = eval_str(s);
    compare (&exp, &e)
}


#[test]
fn test_env_1 () {
    let s = "(defmacro defn (name args body)
                            `(def ,name
                                  (lambda ,args
                                          ,body)))
             (defn f (x) (_+ x 3.0))
             (def x 2)
             (f x)";
    let exp:Expr = Expr::Float(5.0);
    let e = eval_str(s);
    compare (&exp, &e)
}

#[test]
fn test_env_2 () {
    let s = "(defmacro defn (name args body)
                            `(def ,name
                                  (lambda ,args
                                          ,body)))
             (defn add (x) (lambda (y) (_+ x y)))
             (def f (add 3.0))
             (def x 2)
             (f 2)";
    let exp:Expr = Expr::Float(5.0);
    let e = eval_str(s);
    compare (&exp, &e)
}
             

             
