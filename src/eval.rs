use read::Expr;

use std::rc::Rc;
use std::collections::HashMap;


#[derive(Clone,Debug)]
pub struct Context {
    expr: Rc<Expr>,
    env: HashMap<String,Rc<Expr>>
}

impl Context {
    pub fn new(expr: Expr) -> Context {
        Context {
            expr: Rc::new(expr),
            env: HashMap::new()
        }
    }

    pub fn lookup(&self, ident: &String) -> Rc<Expr> {
        match self.env.get (ident) {
            None => panic! ("Lookup: variable not found in environment"),
            Some(x)  => (*x).clone()
        }
    }

    pub fn add_env(&self, ident:String, expr:Rc<Expr>) -> Context {
        let mut context = self.clone();
        context.env.insert(ident, expr);
        context
    }

    fn eval_cons(&self, e1:Rc<Expr>,e2:Rc<Expr>) -> Context {
        match *e1 {
            Expr::Ident(_) => panic! ("FnCall not implemented"),
            Expr::Lambda(_,_) => panic! ("FnCall not implemented"),
            Expr::Macro(_,_) => panic! ("Macro not implemented"),
            _ => panic! ("Eval error: not a function")
        }
    }

    pub fn eval(&self) -> Context {
        match *self.expr {
            Expr::Nil => self.clone(),
            Expr::Integer(_) => self.clone(),
            Expr::Float(_) => self.clone(),
            Expr::String(_) => self.clone(),
            Expr::Quote(ref e) => {
                let mut c = self.clone();
                c.expr = e.clone();
                c
            },
            Expr::Ident(ref s) => {
                let e = self.lookup(s);
                let mut c = self.clone();
                c.expr = e;
                c.eval()
            },
            Expr::Cons(ref e1, ref e2) => self.eval_cons(e1.clone(), e2.clone()),
            _ => panic! ("Not implemented")
        }
    }
}


