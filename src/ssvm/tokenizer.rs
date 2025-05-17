use core::panic;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Var, 
    Assign,
    Define, 
    DefineOp, 
    CTLSPEC,
    CTLEXPR(String),
    Init,
    Next,
    Case,
    Esae,
    CaseDefault,
    Id(String),
    Num(u32),
    True,
    False,
    Colon,
    Semicolon,
    Comma,
    DotDot,
    Equal,
    LBrace,
    RBrace,
    LParen,
    RParen,
    In,
}

fn skip_whitespace<I>(chars: &mut std::iter::Peekable<I>)
where
    I: Iterator<Item = char>,
{
    while let Some(&c) = chars.peek() {
        if c == ' ' || c == '\n' || c == '\t' {
            chars.next();
        } else {
            break;
        }
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => { chars.next(); }, // 跳过空白
            '(' => { tokens.push(Token::LParen); chars.next(); },
            ')' => { tokens.push(Token::RParen); chars.next(); },
            '{' => { tokens.push(Token::LBrace); chars.next(); },
            '}' => { tokens.push(Token::RBrace); chars.next(); },
            ',' => { tokens.push(Token::Comma); chars.next(); },
            ';' => { tokens.push(Token::Semicolon); chars.next(); },
            '=' => { tokens.push(Token::Equal); chars.next(); }
            '.' => { tokens.push(Token::DotDot); chars.next(); chars.next(); }
            '_' => { tokens.push(Token::CaseDefault); chars.next(); },
            ':' => { 
                chars.next(); 
                let tc = chars.peek().unwrap();
                match tc {
                    '=' => {
                        tokens.push(Token::DefineOp);
                        chars.next();
                    },
                    _ => {
                        tokens.push(Token::Colon);
                    }
                }
            },
            _ => {
                if c.is_ascii_digit() {
                    let mut num = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() {
                            num.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let t: u32 = num.parse().unwrap();
                    tokens.push(Token::Num(t));
                }
                else if c.is_alphabetic() {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() {
                            ident.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match ident.to_ascii_lowercase().as_str() {
                        "var" => tokens.push(Token::Var),
                        "assign" => tokens.push(Token::Assign),
                        "define" => tokens.push(Token::Define),
                        "ctlspec" => {
                            tokens.push(Token::CTLSPEC);
                            skip_whitespace(&mut chars);
                            // 读入一整行，给ctl parser
                            let mut ctl_expr = String::new();
                            while let Some(&ch) = chars.peek() {
                                if ch != '\n' && ch != '\r' { // 直到行尾
                                    ctl_expr.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            tokens.push(Token::CTLEXPR(ctl_expr));
                        },
                        "true" => tokens.push(Token::True),
                        "false" => tokens.push(Token::False),
                        "init" => tokens.push(Token::Init),
                        "next" => tokens.push(Token::Next),
                        "case" => tokens.push(Token::Case),
                        "esae" => tokens.push(Token::Esae),
                        "in" => tokens.push(Token::In),
                        _ => tokens.push(Token::Id(ident)),
                    }
                } else {
                    panic!("Unknown Char {}", c );
                }
            }
        }
    }

    tokens
}