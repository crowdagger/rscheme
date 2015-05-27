    // rscheme -- a scheme interpreter written in Rust
    // Copyright (C) {2015) Elizabeth Henry <liz.henry@ouvaton.org>

    // This program is free software; you can redistribute it and/or modify
    // it under the terms of the GNU General Public License as published by
    // the Free Software Foundation; either version 2 of the License, or
    // (at your option) any later version.

#[macro_use]
extern crate log;

mod lexer;
mod read;
mod eval;
mod expr;
mod init;

#[cfg(test)]
mod tests;

use lexer::Token;
use lexer::Lexer;

use std::io::{self,BufRead};
use std::io::Write;

fn repl() {
    let mut c = eval::Context::new();
    c = c.eval_file("data/init.scm");

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    let mut tokens:Vec<Token> = vec!();
    let mut n_par = 0;
    loop {
        if n_par == 0 {
            print!("=> ");
            let r = stdout.flush();
            match r {
                Ok(_) => (),
            Err(_) => {
                error!("Error flushing stdout. abort");
                break;
            }
            }
        }
        
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let cs = line.chars().collect();
        {
            let mut l = Lexer::new(&cs,&mut tokens);
            l.with_n_par(n_par);
            n_par = l.tokenize();

        }
        if n_par == 0 {
            let es = read::read(&tokens);
            tokens = vec!();
            for e in es {
                c = c.eval_expr(e.clone());
                if c.error {
                    c.error = false;
                    break;
                } else {
                    println!("{}", c.expr);
                    info!("{:?}", c.env)
                }
            }
        } else {
            continue;
        }
    }
}

fn main() {
    init::init();
    repl();
}

