use expr::Expr;
use read;

use std::rc::Rc;
use std::collections::HashMap;

const RESERVED_IDENTS:&'static[&'static str] = &[
    "print-debug",
    "defmacro",
    "_cons",
    "lambda",
    "eval",
    "def",
    "if",
    "_+",
    "_-",
    "_*",
    "_/",
    "_=",
    "_car",
    "_cdr"];

fn is_reserved_ident (s: &str) -> bool {
    for i in RESERVED_IDENTS {
        if s == *i {
            return true;
        }
    }
    return false;
}

fn is_equal (e1:Rc<Expr>, e2:Rc<Expr>) -> bool
{
    match *e1 {
        Expr::Integer(x1) => match *e2 {
            Expr::Integer(x2) => x1 == x2,
            _ => false
        },
        Expr::Float(x1) => match *e2 {
            Expr::Float(x2) => x1 == x2,
            _ => false
        },
        Expr::Nil => match *e2 {
            Expr::Nil => true,
            _ => false
        },
        Expr::Ident(ref s1) => match *e2 {
            Expr::Ident(ref s2) => s1 == s2,
            _ => false
        },
        Expr::String(ref s1) => match *e2 {
            Expr::String(ref s2) => s1 == s2,
            _ => false
        },
        Expr::Quote(ref x1) => match *e2 {
            Expr::Quote(ref x2) => is_equal(x1.clone(), x2.clone()),
            _ => false
        },
        Expr::Cons(ref car1, ref cdr1) => match *e2 {
            Expr::Cons(ref car2, ref cdr2) => is_equal(car1.clone(), car2.clone())
                && is_equal(cdr1.clone(), cdr2.clone()),
            _ => false
        },
        _ => false
    }
}

// Merge two environments (= hashmaps)
fn merge_envs (x:HashMap<String,Rc<Expr>>, y:HashMap<String,Rc<Expr>>) -> HashMap<String,Rc<Expr>>
{
    let mut res = x.clone();
    for (s,e) in y.iter() {
        res.insert(s.clone(),e.clone());
    }
    res
}
        

#[derive(Clone,Debug)]
pub struct Context {
    pub expr: Rc<Expr>,
    env: HashMap<String,Rc<Expr>>,
    error: bool
}

impl Context {
    pub fn new() -> Context {
        Context {
            expr: Rc::new(Expr::Nil),
            env: HashMap::new(),
            error: false
        }
    }

    fn error(&self) -> Context {
        let mut c = self.set_expr (Expr::Nil);
        c.error = true;
        c
    }

    fn error_str(&self, s:&str)-> Context {
        println!("{}", s);
        self.error()
    }

    fn has_error(&self) -> bool {
        self.error
    }
    
    pub fn set_expr(&self, expr: Expr) -> Context {
        let mut c = self.clone();
        c.expr = Rc::new(expr);
        c
    }

    pub fn lookup(&self, ident: &String) -> Rc<Expr> {
        match self.env.get (ident) {
            None => {
                println!("Lookup: variable {} not found in environment", ident);
                Rc::new(Expr::Nil)
            },
            Some(x)  => (*x).clone()
        }
    }

    pub fn add_env(&self, ident:String, expr:Rc<Expr>) -> Context {
        if is_reserved_ident (&ident) {
            println!("Keyword {} is reserved", ident);
            self.error()
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
                                    _ => self.error_str("ill-formed if")
                                },
                            _ => self.error_str ("ill-formed if")
                        },
                    _ => self.error_str ("ill-formed if"),
                },
            _ => self.error_str ("ill-formed if")
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
                    _ => self.error_str ("Too many args")
                },
            _ => self.error_str ("Arg is not a cons")
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
                                c = self.clone();
                                c.expr = (*e2).clone();
                                let c = c.eval();
                                let r2 = c.expr.clone();
                                (r1, r2, c.clone())
                            },
                            _ => (Rc::new(Expr::Nil), Rc::new(Expr::Nil), self.error_str("ill-formed operator: too many args"))
                        }
                    },
                    _ => (Rc::new(Expr::Nil), Rc::new(Expr::Nil), self.error_str("ill-formed operator: too many args")),
                },
            _ => (Rc::new(Expr::Nil), Rc::new(Expr::Nil), self.error_str("ill-formed operator: too many args"))
        }        
    }

    fn eval_equal(&self, e:Rc<Expr>) -> Context {
        let (r1, r2, mut c) = self.pre_eval_2(e);
        if c.has_error() {
            c
        } else {
            if is_equal (r1, r2) {
                c.expr = Rc::new(Expr::Ident("t".to_string()));
            } else {
                c.expr = Rc::new(Expr::Nil);
            }
            c
        }
    } 

    fn eval_plus(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Integer(x1 + x2),
                Expr::Float(x2) => Expr::Float((x1 as f64) + x2),
                _ => return self.error_str("Eval error in +: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 + (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 + x2),
                _ => return self.error_str("Eval error in +: invalid types for arguments")
            },
            _ => return self.error_str ("Eval error in +: invalid types for arguments")
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
                _ => return self.error_str("Eval error in -: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 - (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 - x2),
                _ => return self.error_str("Eval error in -: invalid types for arguments")
            },
            _ => return self.error_str("Eval error in -: invalid types for arguments")
        };

        let mut new_c = c.clone();
        new_c.expr = Rc::new(expr);
        new_c
    }

    fn eval_mul(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let expr:Expr = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => {
                    Expr::Integer(x1 * x2)
                },
                Expr::Float(x2) => Expr::Float((x1 as f64) * x2),
                _ => return self.error_str("Eval error in *: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 * (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 * x2),
                _ => return self.error_str("Eval error in *: invalid types for arguments")
            },
            _ => return self.error_str("Eval error in *: invalid types for arguments")
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
                _ => return self.error_str("Eval error in /: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => Expr::Float(x1 / (x2 as f64)),
                Expr::Float(x2) => Expr::Float(x1 / x2),
                _ => return self.error_str("Eval error in /: invalid types for arguments")
            },
            _ => return self.error_str("Eval error in /: invalid types for arguments")
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
                            _ => return self.error_str("Too many arguments to def")
                        },
                    _ => return self.error_str("Wrong number of arguments to def")
                },
            _ => return self.error_str("Wrong number of argumets to def")
        }
        
        match *r1 {
            Expr::Ident(ref s) => {
                let mut c = new_c.add_env(s.clone(), r2.clone());
                c.expr = r2.clone();
                c
            }
            _ => self.error_str("def must take an ident as first parameter")
        }
    }

    fn eval_eval(&self, e:Rc<Expr>) -> Context {
        let c = self.pre_eval_1(e);
        if c.has_error() {
            c
        } else {
            c.eval()
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
            _ => self.error_str ("Error: car must take a list")
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
            _ => self.error_str ("Error: cdr must take a list")
        }
    }

    fn eval_cons (&self, e:Rc<Expr>) -> Context {
        let (r1,r2, mut c) = self.pre_eval_2(e);
        c.expr = Rc::new(Expr::Cons(r1.clone(),r2.clone()));
        c
    }

    fn eval_defmacro (&self, e:Rc<Expr>) -> Context {
        let name:Rc<Expr>;
        let body:Rc<Expr>;
        let args:Rc<Expr>;
        
        match *e {
            Expr::Cons (ref n, ref r) => match **r {
                Expr::Cons (ref a, ref r) => match **r {
                    Expr::Cons (ref b, ref r) => match **r {
                        Expr::Nil => {
                            name = n.clone();
                            body = b.clone();
                            args = a.clone();
                        },
                        _ => return self.error_str("Too many arguments to defmacro")
                    },
                    _ => return self.error_str("Wrong arguments for defmacro")
                },
                _ => return self.error_str("Wrong arguments for defmacro")
            },
            _ => return self.error_str("Wrong arguments for defmacro")
        }

        let n:String = match *name {
            Expr::Ident(ref s) => s.clone(),
            _ => return self.error_str("Error: macro name is not an ident")
        };

        // todo check that args are all idents
        let c = self.set_expr(Expr::Macro(args.clone(), body.clone()));
        c.add_env(n, c.expr.clone())
    }

    fn eval_lambda (&self, e:Rc<Expr>) -> Context {
        let body:Rc<Expr>;
        let args:Rc<Expr>;
        
        match *e {
            Expr::Cons(ref a, ref r) =>
                match **r {
                    Expr::Cons (ref b, ref r) =>
                        match **r {
                            Expr::Nil => {
                                args = a.clone();
                                body = b.clone();
                            },
                            _ => return self.error_str("Too many arguments to lambda")
                        },
                    _ => return self.error_str ("Wrong arguments to lambda")
                },
            _ => return self.error_str ("Wrong arguments to lambda")
        }
        // todo check that args are all idents
        // todo: do better than duplicate env each time...
//        self.set_expr (Expr::Lambda(args,body, self.env.clone()))
        self.set_expr (Expr::Lambda(args,body, HashMap::new()))
    }


    /// Check args of a function or macro call, make them correspond and add them to environment    
    fn eval_fn_args (&self,
                     args_name:Rc<Expr>,
                     args:Rc<Expr>,
                     is_macro:bool,
                     old_c:&Context) -> Context {
        match *args_name {
            Expr::Nil => match *args {
                Expr::Nil => self.clone(), // no args in both cases
                _ => self.error_str("Error in function call: number of arguments don't match")
            },
            Expr::Cons(ref a1, ref r1) => match *args {
                Expr::Cons(ref a2, ref r2) => {
                    match **a1 {
                        Expr::Ident(ref s) => { // it matches, so we do our stuff
                            let mut c = old_c.clone();
                            c.expr = a2.clone();
                            let c = if is_macro {c} else {c.eval()}; //WRONG ENV TO EVAL THIS
                            let v = c.expr.clone(); 
                            let c = self.add_env(s.clone(), v);
                            c.eval_fn_args(r1.clone(),r2.clone(), is_macro, old_c)
                        },
                        _ => self.error_str("Argument name is not an ident!")
                    }
                },
                _ => self.error_str("Error in function call: number of arguments don't match")
            },
            _ => self.error_str("Fn arg names must be a list!")
        }
    }

    fn eval_macro (&self,
                   args_name:Rc<Expr>,
                   body:Rc<Expr>,
                   args:Rc<Expr>) -> Context {
        let mut c = self.eval_fn_args(args_name, args, true, self);
        if c.has_error() {
            self.error()
        } else {
            c.expr = body;
            let mut res = c.eval();
            res.env = self.env.clone();
            // for debug
            println!("Debug: macroexpand gives\n{}", res.expr.clone());
            res.eval()
        }
    }

    fn eval_fncall (&self,
                    args_name:Rc<Expr>,
                    body:Rc<Expr>,
                    args:Rc<Expr>,
                    env:HashMap<String,Rc<Expr>>) -> Context {
        let mut c = self.clone();
        c.env = merge_envs(c.env, env);
        let mut c = c.eval_fn_args (args_name, args, false, self);
        if c.has_error() {
            self.error()
        } else {
            c.expr = body;
            let mut res = c.eval();
            res.env = self.env.clone();
            res
        }
    }

    fn eval_quasiquote (&self) -> Context {
        match *self.expr {
            Expr::Unquote(ref e) => {
                let mut c = self.clone();
                c.expr = e.clone();
                c.eval()
            },
            Expr::Cons(ref car, ref cdr) => {
                let mut c = self.clone();
                c.expr = car.clone();
                c = c.eval_quasiquote();
                let car = c.expr.clone();
                c.expr = cdr.clone();
                c = c.eval_quasiquote();
                let cdr = c.expr.clone();
                c.set_expr(Expr::Cons(car,cdr))
            },
            _ => self.clone()
        }
    }

    fn eval_print_debug(&self, e:Rc<Expr>) -> Context {
        let c = self.pre_eval_1(e);
        println!("{:?}", c.expr.clone());
        c.set_expr(Expr::Nil)
    }
        
        
    fn eval_list_ident(&self, ident:String, e2:Rc<Expr>) -> Context {
        match ident.as_ref() {
            "if" => self.eval_if(e2),
            "_+" => self.eval_plus(e2),
            "_-" => self.eval_sub(e2),
            "_/" => self.eval_div(e2),
            "_*" => self.eval_mul(e2),
            "_=" => self.eval_equal(e2),
            "def" => self.eval_def(e2),
            "_car" => self.eval_car(e2),
            "_cdr" => self.eval_cdr(e2),
            "_cons" => self.eval_cons(e2),
            "lambda" => self.eval_lambda(e2),
            "eval" => self.eval_eval(e2),
            "print-debug" => self.eval_print_debug(e2),
            "defmacro" => self.eval_defmacro(e2),
            _ => self.eval_list (self.lookup(&ident), e2)
        }
    }

    fn eval_list(&self, e1:Rc<Expr>,e2:Rc<Expr>) -> Context {
        match *e1 {
            Expr::Ident(ref str) => self.eval_list_ident(str.clone(),e2),
            Expr::Lambda(ref args, ref body, ref env) => self.eval_fncall (args.clone(), body.clone(), e2.clone(), env.clone()),
            Expr::Cons(_,_) => {
                let mut c = self.clone();
                c.expr = e1.clone();
                let c = c.eval();
                let e = c.expr.clone();
                c.eval_list(e,e2)
            },
            Expr::Macro(ref args,ref body) => self.eval_macro(args.clone(), body.clone(), e2.clone()),
            _ => self.error_str("Invalid argument in first place of evaluated list")
        }
    }

    pub fn eval_expr(&self, expr:Rc<Expr>) -> Context {
        let mut c = self.clone();
        c.expr = expr.clone();
        c.eval()
    }

    pub fn eval_file(&self, file:&str) -> Context {
        let es = read::read_file(file);
        let mut c = self.clone();
        for e in es {
            c = c.eval_expr(e.clone());
        }
        c
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
            Expr::Quasiquote(ref e) => {
                let mut c = self.clone();
                c.expr = e.clone();
                c.eval_quasiquote()
            }
            Expr::Ident(ref s) => {
                let e = self.lookup(s);
                let mut c = self.clone();
                c.expr = e;
                c
                    //c.eval()
            },
            Expr::Cons(ref e1, ref e2) => self.eval_list(e1.clone(), e2.clone()),
            _ => self.clone()
        }
    }
}


