mod lexer;
mod read;
mod eval;
mod expr;

use std::io::{self,BufRead};
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut c = eval::Context::new();
    c = c.eval_file("src/init.scm");
//    println!("{:?}", c);
    
    
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
            println!("{}", c.expr);
        }
    }
}

