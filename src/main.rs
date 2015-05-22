mod lexer;

fn main() {
    let s:&str = "(car    (cdr   (cons   1   (cons 2 
()))))";
    println!("{:?}", lexer::tokenize(s));
}
