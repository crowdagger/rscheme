mod lexer;

fn main() {
    let s:&str = "(car    (cdr   (cons   1   (cons \"a\" '(3.0 4 5)))))";
    let o = lexer::tokenize (s);
    match o {
        None => println! ("First pass failed"),
        Some(v) => println!("{:?}", v)
    }
}
