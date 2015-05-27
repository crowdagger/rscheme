    // rscheme -- a scheme interpreter written in Rust
    // Copyright (C) {2015) Elizabeth Henry <liz.henry@ouvaton.org>

    // This program is free software; you can redistribute it and/or modify
    // it under the terms of the GNU General Public License as published by
    // the Free Software Foundation; either version 2 of the License, or
    // (at your option) any later version.


use expr::Expr;
use read;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;

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
    "_<",
    "_>",
    "_car",
    "_cdr",
    "_&",
    "_nil?",
    "_lambda?",
    "_macro?",
    "_integer?",
    "_float?",
    "_ident?",
    "_string?",
    "_quote?",
    "_unquote?",
    "_quasiquote?",
    "_list?",
        "_str",
    "_print"];

fn is_reserved_ident (s: &str) -> bool {
    for i in RESERVED_IDENTS {
        if s == *i {
            return true;
        }
    }
    return false;
}

// Merge two environments (= hashmaps)
fn merge_envs (x:&mut HashMap<String,Rc<Expr>>, y:&Option<HashMap<String,Rc<Expr>>>) 
{
    //    let mut res = x.clone();
    match *y {
        None => (),
        Some(ref h) =>  {
            for (s,e) in h {
                x.insert(s.clone(),e.clone());
            }
        }
    }
}
        

#[derive(Clone,Debug)]
pub struct Context {
    pub expr: Rc<Expr>,
    pub env: Rc<RefCell<HashMap<String,Rc<Expr>>>>,
    pub global_env: Rc<RefCell<HashMap<String,Rc<Expr>>>>,
    pub error: bool
}

impl Context {
    pub fn new() -> Context {
        Context {
            expr: Rc::new(Expr::Nil),
            env: Rc::new(RefCell::new((HashMap::new()))),
            global_env: Rc::new(RefCell::new((HashMap::new()))),
            error: false
        }
    }

    fn error(&self) -> Context {
        let mut c = self.set_expr (Expr::Nil);
        c.error = true;
        c
    }

    fn error_str(&self, s:&str)-> Context {
        error!("{}", s);
        self.error()
    }

    pub fn has_error(&self) -> bool {
        self.error
    }
    
    pub fn set_expr(&self, expr: Expr) -> Context {
        let mut c = self.clone();
        c.expr = Rc::new(expr);
        c
    }

    pub fn set_rcexpr(&self, expr:Rc<Expr>) -> Context {
        let mut c = self.clone();
        c.expr = expr.clone();
        c
    }

    pub fn lookup(&self, ident: &String) -> Context {
        let env = self.env.borrow();
        let global_env = self.global_env.borrow();
        match env.get(ident) {
            None => {
                match global_env.get(ident) {
                    None => {
                        error!("Lookup: variable {} not found in environment", ident);
                        self.error()
                    },
                    Some(x) => self.set_rcexpr(x.clone())
                }
            },
            Some(x)  => self.set_rcexpr(x.clone())
        }
    }

    // returns a context with a duplicated env hashmap, so inserting into it won't
    // fuck up parent context
    pub fn dup_env(&self) -> Context {
        let mut c = self.clone();
        let newhash:HashMap<String,Rc<Expr>> = c.env.borrow_mut().clone();
        c.env = Rc::new(RefCell::new(newhash));
        c
    }
    
    pub fn add_env(&self, ident:String, expr:Rc<Expr>) -> Context {
        if is_reserved_ident (&ident) {
            error!("Keyword {} is reserved", ident);
            self.error()
        } else {
            let c = self.dup_env();
            c.env.borrow_mut().insert(ident, expr);
            c
        }
    }

    pub fn add_global(&self, ident:String, expr:Rc<Expr>) -> Context {
        if is_reserved_ident (&ident) {
            error!("Keyword {} is reserved", ident);
            self.error()
        } else {
            let mut global_env = self.global_env.borrow_mut();
            global_env.insert(ident, expr);
            self.clone()
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
            if r1 == r2 {
                c.expr = Rc::new(Expr::Ident("t".to_string()));
            } else {
                c.expr = Rc::new(Expr::Nil);
            }
            c
        }
    }

    // Display the value of the expression
    fn eval_print(&self, e:Rc<Expr>) -> Context {
        let c = self.pre_eval_1(e);
        match *c.expr {
            Expr::String(ref s) => print!("{}", s),
            _ => print!("{}", *c.expr)
        }
        self.set_expr(Expr::Nil)
    }
    
    // Concatenate two values, as a sring
    fn eval_str(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,_) = self.pre_eval_2(e);
        let s1 = match *r1 {
            Expr::String(ref s) => format!("{}", s),
            _ => format!("{}", r1)
        };
        let s2 = match *r2 {
            Expr::String(ref s) => format!("{}", s),
            _ => format!("{}", r2)
        };
        let s = format!("{}{}",s1, s2);
        self.set_expr(Expr::String(s.to_string()))
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

    fn eval_gt(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let is_lt = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => x1 > x2,
                Expr::Float(x2) => (x1 as f64) > x2,
                _ => return self.error_str("Eval error in >: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => x1 > (x2 as f64),
                Expr::Float(x2) => x1 > x2,
                _ => return self.error_str("Eval error in >: invalid types for arguments")
            },
            Expr::String(ref s1) => match *r2 {
                Expr::String(ref s2) => s1 > s2,
                _ => return self.error_str("Eval error in >: invalid types for arguments")
            },
            _ => return self.error_str("Eval error in >: invalid types for arguments")
        };

        let expr = if is_lt {
            Expr::Ident("t".to_string())
        } else {
            Expr::Nil
        };
        
        c.set_expr(expr)
    }

    fn eval_lt(&self, e:Rc<Expr>) -> Context {
        let (r1,r2,c) = self.pre_eval_2(e);

        let is_lt = match *r1 {
            Expr::Integer(x1) => match *r2 {
                Expr::Integer(x2) => x1 < x2,
                Expr::Float(x2) => (x1 as f64) < x2,
                _ => return self.error_str("Eval error in <: invalid types for arguments")
            },
            Expr::Float(x1) => match *r2 {
                Expr::Integer(x2) => x1 < (x2 as f64),
                Expr::Float(x2) => x1 < x2,
                _ => return self.error_str("Eval error in <: invalid types for arguments")
            },
            Expr::String(ref s1) => match *r2 {
                Expr::String(ref s2) => s1 < s2,
                _ => return self.error_str("Eval error in <: invalid types for arguments")
            },
            _ => return self.error_str("Eval error in <: invalid types for arguments")
        };

        let expr = if is_lt {
            Expr::Ident("t".to_string())
        } else {
            Expr::Nil
        };
        
        c.set_expr(expr)
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
                let mut c = new_c.add_global(s.clone(), r2.clone());
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
        c.add_global(n, c.expr.clone())
    }

    // Collects all idents used by a lambda,
    fn collect_idents(&self,
                      expr:&Expr,
                      ids: &mut HashSet<String>,
                      ignore: &HashSet<String>,
                      quote: bool) {
        match *expr {
            Expr::Ident(ref s) => {
                if !quote && !ignore.contains(s) &&
                    !is_reserved_ident(s) && !self.global_env.borrow().contains_key(s) {
                        ids.insert(s.clone());
                }
            },
            Expr::Cons(ref e1, ref e2) => {
                match **e1 {
                    Expr::Ident(ref s) => {
                        if "lambda".to_string() == s.clone() {
                            if !quote {
                                // not our problem (for now)
                            } else {
                                self.collect_idents(e2, ids, ignore, quote);
                            }
                        } else {
                            self.collect_idents(e1, ids, ignore, quote);
                            self.collect_idents(e2, ids, ignore, quote);
                        }
                    },
                    _ => {
                        self.collect_idents(e1, ids, ignore, quote);
                        self.collect_idents(e2, ids, ignore, quote);
                    }
                }
            },
            Expr::Quasiquote(ref e) => {
                self.collect_idents(e, ids, ignore, true);
            },
            Expr::Unquote(ref e) => {
                self.collect_idents(e, ids, ignore, false);
            },
            _ => ()
        }
    }

        
    
    // This method has two purposes:
    // 1) Checks that all args are valid (i.e) are idents
    // 2) Add idents in a vector so we don't need to add them to closure env
    fn lambda_verify_args (&self,
                           e:&Expr,
                           v:&mut HashSet<String>) -> bool {
        match *e {
            Expr::Cons(ref e1, ref e2) => self.lambda_verify_args (e1, v) &&
                self.lambda_verify_args (e2, v),
            Expr::Ident(ref s) => {
                v.insert(s.clone());
                true
            },
            Expr::Nil => true,
            _ => {
                error!("Error in lambda declaration: invalid form for args (must be a list of idents");
                false
            }
        }
    }

    fn eval_lambda (&self, e:Rc<Expr>) -> Context {
        let body:Rc<Expr>;
        let args:Rc<Expr>;
        let mut name:Option<String> = None;

        match *e {
            Expr::Cons(ref a, ref r) =>
                match **r {
                    Expr::Cons (ref b, ref r) =>
                        match **r {
                            Expr::Nil => {
                                args = a.clone();
                                body = b.clone();
                            },
                            Expr::Cons(ref c, ref r) =>
                                match **r {
                                    Expr::Nil => {
                                        match **a {
                                            Expr::Ident(ref s) =>  {
                                                name = Some(s.clone());
                                                args = b.clone();
                                                body = c.clone();
                                            }
                                            _ => {
                                                error!("Error in, lambda for name, expected ident, got {:?}", a);
                                                return self.error();
                                            }
                                        }
                                            
                                    },
                                    _ => return self.error_str("Too many arguments to lambda")
                                },
                            _ => return self.error_str("Wrong arguments to lambda")
                        },
                    _ => return self.error_str ("Wrong arguments to lambda")
                },
            _ => return self.error_str ("Wrong arguments to lambda")
        }

        // Collect all idents in body, except those in args,
        // and adds them to an its environment
        let mut ignore:HashSet<String> = HashSet::new();
        if self.lambda_verify_args (&*args, &mut ignore) {
            match name {
                Some(ref s) => {
                    ignore.insert(s.clone());
                }
                None => ()
            }
            let mut ids:HashSet<String> = HashSet::new();
            self.collect_idents(&*body, &mut ids, &ignore, false);
            for i in ignore {
                ids.remove(&i);
            }
            let env = if ids.is_empty() {
                None
            } else {
                let mut e:HashMap<String,Rc<Expr>> = HashMap::new();
                for k in ids {
                    let c = self.lookup(&k);
                    let v = c.expr.clone();
                    if c.has_error() {
                        error!("Lambda depends on ident {} but it can't be found in this context", &k);
                        return self.error();
                     } else {
                        e.insert(k.clone(),v);
                    }
                }
                Some(e)
            };
            let l_name:String = match name {
                None => "".to_string(),
                Some(s) => s.clone()
            };
            self.set_expr (Expr::Lambda(l_name, args,body, env))
        } else {
            self.error()
        }
    }


    // Eval all elements in a list, returning the list of all eval'ed elements
    fn eval_all_in_list(&self, expr:Rc<Expr>) -> Context
    {
        match *expr {
            Expr::Nil => self.set_expr(Expr::Nil),
            Expr::Cons(ref e, ref r) => {
                let mut c = self.clone();
                c.expr = e.clone();
                c = c.eval();
                let evaled_car = c.expr;
                c = self.eval_all_in_list(r.clone());
                let evaled_cdr = c.expr;
                self.set_expr(Expr::Cons(evaled_car, evaled_cdr))
            },
            _ => self.error_str("Wrong list for eval_all_in_list")
        }
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
                _ => self.error_str("1Error in function call: number of arguments don't match")
            },
            Expr::Cons(ref a1, ref r1) => {
                info!("{:?}", a1);
                if let Expr::Ident(ref s) = **a1 {
                    if s == "&" { // case for catchall argument name
                        if let Expr::Cons(ref catchall, ref r1_bis) = **r1 {
                                    match **r1_bis {
                                        Expr::Nil => {
                                            match **catchall {
                                                Expr::Ident(ref s) => { 
                                                    let v = if !is_macro {
                                                        let c = self.eval_all_in_list(args.clone());
                                                        c.expr
                                                    } else {
                                                        args.clone()
                                                    };
                                                    info!("args: {:?}", args);
                                                    info!("evalued {} to {:?}", s, v);
                                                    return self.add_env(s.clone(), v);
                                                },
                                                _ => return self.error_str("Catch all is not an ident")
                                            }},
                                        _ => {
                                            return self.error_str("Catch all argument must be the last of the list")
                                        }}}}}
                match *args{
                    Expr::Cons(ref a2, ref r2) => {
                        match **a1 {
                            Expr::Ident(ref s) => {
                                let mut c = old_c.clone();
                                c.expr = a2.clone();
                                c = if is_macro {c} else {c.eval()}; //WRONG ENV TO EVAL THIS ?
                                let v = c.expr.clone();
                                info!("evalued {} to {:?}", s, v);
                                c = self.add_env(s.clone(), v);
                                c.eval_fn_args(r1.clone(),r2.clone(), is_macro, old_c)
                            },
                            _ => self.error_str("Argument name is not an ident!")                            
                        }},
                    _ => self.error_str("2Error in function call: number of arguments don't match")
                    }},
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
            info!("Debug: macroexpand gives\n{}", res.expr.clone());
            res.eval()
        }
    }

    fn eval_fncall (&self,
                    name:String,
                    args_name:Rc<Expr>,
                    body:Rc<Expr>,
                    args:Rc<Expr>,
                    env:&Option<HashMap<String,Rc<Expr>>>) -> Context {
        let mut c = self.clone();
        if !name.is_empty() {
            c = c.add_env(name.clone(), Rc::new(Expr::Lambda(name.clone(),args_name.clone(),body.clone(),None)));
        } else {
            c = c.dup_env();
        }
        merge_envs(&mut c.env.borrow_mut(), env);
 
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

    // Checks the types of an expression
    fn eval_type_check(&self, t:&str, e:Rc<Expr>) -> Context {
        let c = self.pre_eval_1(e);
        let e = c.expr.clone();
        let is_ok =  match t {
            "_nil?" => if let Expr::Nil = *e {true} else {false},
            "_list?" => if let Expr::Cons(_,_) = *e {true} else {false},
            "_lambda?" => if let Expr::Lambda(_,_,_,_) = *e {true} else {false},
            "_integer?" => if let Expr::Integer(_) = *e {true} else {false},
            "_float?" => if let Expr::Float(_) = *e {true} else {false},
            "_ident?" => if let Expr::Ident(_) = *e {true} else {false},
            "_string?" => if let Expr::String(_) = *e {true} else {false},
            "_quote?" => if let Expr::Quote(_) = *e {true} else {false},
            "_quasiquote?" => if let Expr::Quasiquote(_) = *e {true} else {false},
            "_unquote" => if let Expr::Unquote(_) = *e {true} else {false},
            _ => panic!("Unrecognized type")
        };
        if is_ok {
            self.set_expr(Expr::Ident("t".to_string()))
        } else {
            self.set_expr(Expr::Nil)
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
            "_<" => self.eval_lt(e2),
            "_>" => self.eval_gt(e2),
            "_str" => self.eval_str(e2),
            "_print" => self.eval_print(e2),
            "_nil?" | "_lambda?" | "_integer?" | "_macro?" | "_float?" 
                | "_ident?" | "_string?" | "_list?" | "_quote?"
                | "_unquote?" | "_quasiquote?" => self.eval_type_check(ident.as_ref(),e2),
            "def" => self.eval_def(e2),
            "_car" => self.eval_car(e2),
            "_cdr" => self.eval_cdr(e2),
            "_cons" => self.eval_cons(e2),
            "lambda" => self.eval_lambda(e2),
            "eval" => self.eval_eval(e2),
            "print-debug" => self.eval_print_debug(e2),
            "defmacro" => self.eval_defmacro(e2),
            _ => {
                let c = self.lookup(&ident);
                if c.error {
                    c
                } else {
                    self.eval_list (c.expr, e2)
                }
            }
        }
    }

    fn eval_list(&self, e1:Rc<Expr>,e2:Rc<Expr>) -> Context {
        match *e1 {
            Expr::Ident(ref str) => self.eval_list_ident(str.clone(),e2),
            Expr::Lambda(ref name, ref args, ref body, ref env) => self.eval_fncall (name.clone(), args.clone(), body.clone(), e2.clone(), env),
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
        c.env = Rc::new(RefCell::new(HashMap::new()));
        c.expr = expr.clone();
        c.eval()
    }

    pub fn eval_file(&self, file:&str) -> Context {
        let es = read::read_file(file);
        let mut c = self.clone();
        for e in es {
            c = c.eval_expr(e.clone());
            if c.error {
                break;
            }
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
                let c = self.lookup(s);
                if c.error {
                    c
                } else {
                    let e = c.expr;
                    self.set_rcexpr(e)
                        //c.eval()
                }
            },
            Expr::Cons(ref e1, ref e2) => self.eval_list(e1.clone(), e2.clone()),
            _ => self.clone()
        }
    }
}


