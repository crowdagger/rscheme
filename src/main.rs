mod lexer;
mod list;
mod read;
mod eval;

use std::rc::Rc;
use std::io::{self,BufRead};
use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut c = eval::Context::new();
    
    loop {
        print!("=> ");
        stdout.flush();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();

        let e = read::read_str(line.as_ref());
        c = c.eval_expr(e);
        println!("{:?}", c.expr);
    }
}

