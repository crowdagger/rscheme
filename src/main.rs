mod lexer;
mod list;
mod read;

fn main() {
    let l:list::List<u32> = list::List::new().cons(2).cons(1);
    let l = l.map(|x| 2 * x);
    println! ("{}", l.count());
    println! ("{:?}", l.car());
    println! ("{:?}", l.cdr().car());
    println! ("{:?}", l.cdr().cdr().car());

    
    let s:&str = "(1 2 (3.0 \"toto\" ) 4)";
//    let s = "(1 2)";
    let o = lexer::tokenize (s);
    match o {
        None => println! ("First pass failed"),
        Some(v) => println!("{:?}", read::read(&*v))
    }
}
