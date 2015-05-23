use read::Expr;

use std::rc::Rc;
use std::collections::HashMap;

const RESERVED_IDENTS:&'static[&'static str] = &[
    "cons",
    "lambda",
    "def",
    "if",
    "+",
    "-",
    "*",
    "/",
    "car",
    "cdr"];

fn is_reserved_ident (s: &str) -> bool {
    for i in RESERVED_IDENTS {
        if s == *i {
            return true;
        }
    }
    return false;
}
        

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
        if is_reserved_ident (&ident) {
            panic!("Use of reserved keyword");
        } else {
            let mut context = self.clone();
            context.env.insert(ident, expr);
            context
        }
    }

    fn eval_if_form (&self, p:Rc<Expr>, t:Rc<Expr>, f:Rc<Expr>) -> Context {
        let mut c = self.clone();
        c.expr = p.clone();
        let c = c.eval();
        match *c.expr {
            Expr::Nil => {
                let mut res = c.clone();
                res.expr = f.clone();
                res.eval()
            },
            _ => { // anything but nil is true
                let mut res = c.clone();
                res.expr = t.clone();
                res.eval()
            }
        }
    }

    fn eval_if (&self, e:Rc<Expr>) -> Context {
        match *e {
            Expr::Cons(ref p, ref r) =>
                match **r {
                    Expr::Cons (ref t, ref r) =>
                        match **r {
                            Expr::Cons (ref f, ref r) =>
                                match **r {
                                    Expr::Nil => self.eval_if_form (p.clone(), t.clone(), f.clone()),
                                    _ => panic!("ill-formed if")
                                },
                            _ => panic! ("ill-formed if")
                        },
                    _ => panic!("ill-formed if"),
                },
            _ => panic!("ill-formed if")
        }
    }

    fn pre_eval_1(&self, e:Rc<Expr>) -> Context {
        match *e {
            Expr::Cons (ref e1, ref r) =>
                match **r {
                    Expr::Nil => {
                        let mut c = self.clone();
                        c.expr = e1.clone();
                        c.eval()
                    },
                    _ => panic!("Too many args")
                },
            _ => panic!("Arg is not a cons")
        }
    }

    fn pre_eval_2(&self, e:Rc<Expr>) -> (Rc<Expr>, Rc<Expr>, Context) {
        match *e {
            Expr::Cons (ref e1, ref r) =>
                match **r {
                    Expr::Cons (ref e2, ref r) => {
                        match **r {
                            Expr::Nil => {
                                let mut c = self.clone();
                                c.expr = (*e1).clone();
                                let mut c = c.eval().clone();
                                let r1 = c.expr.clone();
                                c.expr = (*e2).clone();
                                let c = c.eval();
                                let r2 = c.expr.clone();
                                (r1, r2, c.clone())
                            },
                            _ => panic!("ill-formed operator: too many args")
                        }
                    },
                    _ => panic!("ill-formed operator"),
                },
            _ => panic!("ill-formed operator")
        }        
    }

    fn eval_plus(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Integer(x1 + x2),
                Expr::Float(x2) => Expr::Float((x1 as f64) + x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 + (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 + x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            _ => panic!("Eval error in +: invalid types for arguments")
        };

        let mut new_c = c.clone();
        new_c.expr = Rc::new(expr);
        new_c
    }

    fn eval_sub(&self, e:Rc<Expr>) -> Context {
        //TODO: implement for only one argument
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Integer(x1 - x2),
                Expr::Float(x2) => Expr::Float((x1 as f64) - x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 - (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 - x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            _ => panic!("Eval error in +: invalid types for arguments")
        };

        let mut new_c = c.clone();
        new_c.expr = Rc::new(expr);
        new_c
    }

    fn eval_mul(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Integer(x1 * x2),
                Expr::Float(x2) => Expr::Float((x1 as f64) * x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 * (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 * x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            _ => panic!("Eval error in +: invalid types for arguments")
        };

        let mut new_c = c.clone();
        new_c.expr = Rc::new(expr);
        new_c
    }

    fn eval_div(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Integer(x1 / x2),
                Expr::Float(x2) => Expr::Float((x1 as f64) / x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 / (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 / x2),
                _ => panic!("Eval error in +: invalid types for arguments")
            },
            _ => panic!("Eval error in +: invalid types for arguments")
        };

        let mut new_c = c.clone();
        new_c.expr = Rc::new(expr);
        new_c
    }

    fn eval_def(&self, e:Rc<Expr>) -> Context {
        let r1:Rc<Expr>;
        let r2:Rc<Expr>;
        let mut new_c:Context;

        match *e {
            Expr::Cons(ref e1, ref r) =>
                match **r {
                    Expr::Cons(ref e2, ref r) =>
                        match **r {
                            Expr::Nil => {
                                r1 = e1.clone();
                                let mut c = self.clone();
                                c.expr = e2.clone();
                                new_c = c.eval();
                                r2 = new_c.expr.clone()
                            }
                            _ => panic!("Too many arguments to def")
                        },
                    _ => panic!("Wrong number of arguments to def")
                },
            _ => panic!("Wrong number of argumets to def")
        }
        
        match *r1 {
            Expr::Ident(ref s) => {
                let mut c = new_c.add_env(s.clone(), r2.clone());
                c.expr = r2.clone();
                c
            }
            _ => panic!("def must take an ident as first parameter")
        }
    }

    fn eval_car (&self, e:Rc<Expr>) -> Context {
        let mut c = self.pre_eval_1(e);
        let e = c.expr.clone();
        match *e {
            Expr::Cons (ref car, _) => {
                c.expr = car.clone();
                c
            }
            _ => panic! ("Error: car must take a list")
        }
    }

    fn eval_cdr (&self, e:Rc<Expr>) -> Context {
        let mut c = self.pre_eval_1(e);
        let e = c.expr.clone();
        match *e {
            Expr::Cons (_, ref cdr) => {
                c.expr = cdr.clone();
                c
            }
            _ => panic! ("Error: cdr must take a list")
        }
    }
        
    fn eval_cons_ident(&self, ident:String, e2:Rc<Expr>) -> Context {
        match ident.as_ref() {
            "if" => self.eval_if(e2),
            "+" => self.eval_plus(e2),
            "-" => self.eval_sub(e2),
            "/" => self.eval_div(e2),
            "*" => self.eval_mul(e2),
            "def" => self.eval_def(e2),
            "car" => self.eval_car(e2),
            "cdr" => self.eval_cdr(e2),
            "cons" => panic!("Cons not implemented"),
            "lambda" => panic!("lambda not implemented"),
            _ => panic!("custom call not implemented")
        }
    }

    fn eval_cons(&self, e1:Rc<Expr>,e2:Rc<Expr>) -> Context {
        match *e1 {
            Expr::Ident(ref str) => self.eval_cons_ident(str.clone(),e2),
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


