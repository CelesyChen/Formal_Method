use core::panic;

use crate::ctl::tokenizer::*;
use crate::ctl::ast::*;

pub struct CtlParser {
    tokens: Vec<Token>,
    pos: usize,
}

impl CtlParser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.peek() == Some(&expected) {
            self.next();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        if token.is_some() {
            self.pos += 1;
        }
        token
    }
    
    fn expect(&mut self, expected: &Token) {
        let token = self.next();
        if token.as_ref() != Some(expected) {
            panic!("Expected {:?}, but got {:?} at pos {:?}", expected, token, self.pos);
        }
    }

    pub fn parse (&mut self) -> CtlAst {
        self.parse_i()
    }

    fn parse_i (&mut self) -> CtlAst {
        let mut node = self.parse_o();
        if self.match_token(Token::Implies) {
            let right = self.parse_i();
            node = CtlAst::Implies(Box::new(node), Box::new(right));
        }
        node
    }

    fn parse_o(&mut self) -> CtlAst {
        let mut node = self.parse_a();
        while self.match_token(Token::Or) {
            let right = self.parse_o();
            node = CtlAst::Or(Box::new(node), Box::new(right));
        }
        node
    }

    fn parse_a (&mut self) -> CtlAst {
        let mut node = self.parse_n();
        while self.match_token(Token::And) {
            let right = self.parse_a();
            node = CtlAst::And(Box::new(node), Box::new(right));
        }
        node   
    }

    fn parse_n (&mut self) -> CtlAst {
        if self.match_token(Token::Not) {
            let expr = self.parse_n();
            CtlAst::Not(Box::new(expr))
        } else {
            self.parse_p()
        }
    }

    fn parse_p(&mut self) -> CtlAst {
        match self.peek() {
            Some(Token::AG) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::AG(Box::new(s)) }
            Some(Token::EG) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::EG(Box::new(s)) }
            Some(Token::AX) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::AX(Box::new(s)) }
            Some(Token::EX) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::EX(Box::new(s)) }
            Some(Token::AF) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::AF(Box::new(s)) }
            Some(Token::EF) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); CtlAst::EF(Box::new(s)) }
            Some(Token::AU) => {
                self.next(); // A[
                self.next();
                let left = self.parse();
                self.expect(&Token::Until);
                let right = self.parse();
                self.expect(&Token::RBracket);
                CtlAst::AU(Box::new(left), Box::new(right))
            },
            Some(Token::EU) => {
                self.next(); // E[
                self.next();
                let left = self.parse();
                self.expect(&Token::Until);
                let right = self.parse();
                self.expect(&Token::RBracket);
                CtlAst::EU(Box::new(left), Box::new(right))
            },
            Some(Token::LParen) => {
                self.next();
                let s = self.parse();
                self.expect(&Token::RParen);
                s
            }
            Some(Token::True) => { self.next(); CtlAst::True }
            Some(Token::False) => { self.next(); CtlAst::False }
            Some(Token::Identifier(name)) => { 
                let name = name.clone();
                self.next(); self.next();
                let atom = self.parse_atom();
                CtlAst::Expr(name, atom)
            }
            _ => panic!("Unexpected token in primary expression: {:?}", self.peek())
        }
    }

    fn parse_atom(&mut self) -> Atom {
        match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.next();
                Atom::Id(name)
            },
            Some(Token::Num(num)) => {
                let num = num.clone();
                self.next();
                Atom::Num(num)
            },
            _ => panic!("Unexpected token in atom: {:?}", self.peek())
        }
    }
    
}