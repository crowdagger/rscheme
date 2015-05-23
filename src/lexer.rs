use std::result;

pub type Result = result::Result<Token, &'static str>;

#[derive(Debug)]
pub enum Token {
    Integer (i64),
    Float (f64),
    Ident (String),
    String (String),
    OpeningParen,
    ClosingParen,
    Quote
}

fn read_integer (s: &str) -> Result {
    match s.parse::<i64>() {
        Ok(x) => Ok(Token::Integer(x)),
        Err(_) => Err("Error parsing integer")
    }
}

fn read_float (s: &str) -> Result {
    match s.parse::<f64>() {
        Ok(x) => Ok(Token::Float(x)),
        Err(_) => Err("Error Parsing float")
    }
}
            

fn read_number (s: &str) -> Result {
    let v:Vec<&str> = s.split('.').collect();
    let x = v.len ();
    match x {
        1 => read_integer (v[0]),
        2 => read_float (s),
        _ => Err("Error parsing numerical token (too many dots)")
    }
}

fn read_string (s: &str) -> Result {
    let cs = s.chars();
    let mut s = String::new();
    let mut prev_escape = false;
    let mut quotes = 0;
    
    for c in cs  {
        match c {
            '"' => if prev_escape {
                prev_escape = false;
                s.push('"');
            } else {
                quotes += 1;
                prev_escape = false;
                if quotes >= 2 {
                    break;
                }
            },
            '\\' => if prev_escape {
                prev_escape = false;
                s.push ('\\');
            } else {
                prev_escape = true;
            },
            v => if prev_escape {
                return Err("Parse error in string: invalid escape character");
            } else {
                s.push(v);
            }
        }
    }
    Ok(Token::String(s))
}

fn read_ident (s: &str) -> Result {
    Ok(Token::Ident(s.to_string()))
}

fn read_single_token (s: &str) -> Result {
    match s.chars().nth(0) {
        None => Err("Empty token"),
        Some('(') => Ok(Token::OpeningParen),
        Some(')') => Ok(Token::ClosingParen),
        Some('\'') => Ok(Token::Quote),
        Some('"') => read_string(s),
        Some('0' ... '9') => read_number(s),
        Some(_) => read_ident(s)
    }
}
        
pub fn tokenize(s: &str) -> Option<Vec<Token>> {
    let s = s.replace("(", " ( ")
        .replace(")", " ) ")
        .replace("'", "' ");
    let x: Vec<&str> = s.split(|c:char| c.is_whitespace())
        .filter(|s:&&str| if *s=="" {false} else {true})
        .collect();
//    println!("{:?}", x);
    let mut res: Vec<Token> = vec! ();
    let mut err = false;
    for s in x.iter () {
        let r = read_single_token (s);
        match r {
            Err(s) => {
                println! ("{}", s);
                err = true;
            },
            Ok(t) => res.push (t)
        }
    }
    if err {
        None
    } else {
        Some(res)
    }
}
