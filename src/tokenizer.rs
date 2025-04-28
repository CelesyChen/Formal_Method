#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    And,
    Or,
    Implies,
    Not,
    AG,
    EG,
    AX,
    EX,
    AF,
    EF,
    AU,
    EU,
    Until, //这个其实没什么用,只是吸收掉U而已
    LParen,
    RParen,
    LBracket,
    RBracket,
    Identifier(String),
    True,
    False,
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
            '[' => { tokens.push(Token::LBracket); chars.next(); },
            ']' => { tokens.push(Token::RBracket); chars.next(); },
            '&' => { tokens.push(Token::And); chars.next(); },
            '|' => { tokens.push(Token::Or); chars.next(); },
            '!' => { tokens.push(Token::Not); chars.next(); },
            'T' => { tokens.push(Token::True); chars.next(); },
            'F' => { tokens.push(Token::False); chars.next(); },
            '-' => { 
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Implies);
                } else {
                    panic!("Unexpected character after '-': expected '>'");
                }
            },
            _ => {
                if c.is_alphabetic() {
                    let mut ident = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() {
                            ident.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    match ident.as_str() {
                        "AG" => tokens.push(Token::AG),
                        "EG" => tokens.push(Token::EG),
                        "AX" => tokens.push(Token::AX),
                        "EX" => tokens.push(Token::EX),
                        "AF" => tokens.push(Token::AF),
                        "EF" => tokens.push(Token::EF),
                        "U" => tokens.push(Token::Until),
                        "A" => {
                            skip_whitespace(&mut chars);
                            if let Some(&'[') = chars.peek() {
                                chars.next();
                                tokens.push(Token::AU);
                                tokens.push(Token::LBracket);
                            } else {
                                tokens.push(Token::Identifier(ident));
                            }
                        },
                        "E" => {
                            skip_whitespace(&mut chars);
                            if let Some(&'[') = chars.peek() {
                                chars.next();
                                tokens.push(Token::EU);
                                tokens.push(Token::LBracket);
                            } else {
                                tokens.push(Token::Identifier(ident));
                            }
                        },
                        _ => tokens.push(Token::Identifier(ident)),
                    }
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    tokens
}
