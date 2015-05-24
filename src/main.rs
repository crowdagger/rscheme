#[macro_use]
extern crate log;

mod lexer;
mod read;
mod eval;
mod expr;
mod init;

#[cfg(test)]
mod tests;

use std::io::{self,BufRead};
use std::io::Write;
use std::env;





fn main() {
    init::init();        
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut c = eval::Context::new();
    c = c.eval_file("data/init.scm");
    
    loop {
        print!("=> ");
        let r = stdout.flush();
        match r {
            Ok(_) => (),
            Err(_) => {
                println!("Error flushing stdout. abort");
                break;
            }
        }
                
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let es = read::read_str(line.as_ref());
        for e in es {
            c = c.eval_expr(e.clone());
            if c.error {
                c.error = false;
                break;
            } else {
                println!("{}", c.expr);
            }
        }
    }
}

