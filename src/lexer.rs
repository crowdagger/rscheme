#[derive(Debug)]
pub enum Token<'a> {
    Nil,
    Integer (i64),
    Float (f64),
    Ident (&'a str),
    String (&'a str),
    OpeningParen,
    ClosingParen,
    Quote
}

pub fn tokenize(s: &str) -> Vec<String> {
    let s = s.replace("(", " ( ").replace(")", " ) ");
    let x: Vec<String> = s.split(|c:char| c.is_whitespace())
        .filter(|s:&&str| if *s=="" {false} else {true})
        .map(|s:&str| s.to_string())
        .collect();
    x
}
