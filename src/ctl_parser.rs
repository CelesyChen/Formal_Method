use crate::tokenizer::*;
use crate::ast::*;

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
    

    pub fn parse (&mut self) -> AstNode {
        self.parse_i()
    }

    fn parse_i (&mut self) -> AstNode {
        let mut node = self.parse_o();
        if self.match_token(Token::Implies) {
            let right = self.parse_i();
            node = AstNode::Implies(Box::new(node), Box::new(right));
        }
        node
    }

    fn parse_o(&mut self) -> AstNode {
        let mut node = self.parse_a();
        while self.match_token(Token::Or) {
            let right = self.parse_o();
            node = AstNode::Or(Box::new(node), Box::new(right));
        }
        node
    }

    fn parse_a (&mut self) -> AstNode {
        let mut node = self.parse_n();
        while self.match_token(Token::And) {
            let right = self.parse_a();
            node = AstNode::And(Box::new(node), Box::new(right));
        }
        node   
    }

    fn parse_n (&mut self) -> AstNode {
        if self.match_token(Token::Not) {
            let expr = self.parse_n();
            AstNode::Not(Box::new(expr))
        } else {
            self.parse_p()
        }
    }

    fn parse_p(&mut self) -> AstNode {
        match self.peek() {
            Some(Token::AG) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::AG(Box::new(s)) }
            Some(Token::EG) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::EG(Box::new(s)) }
            Some(Token::AX) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::AX(Box::new(s)) }
            Some(Token::EX) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::EX(Box::new(s)) }
            Some(Token::AF) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::AF(Box::new(s)) }
            Some(Token::EF) => { self.next(); self.expect(&Token::LParen); let s = self.parse(); self.expect(&Token::RParen); AstNode::EF(Box::new(s)) }
            Some(Token::AU) => {
                self.next(); // A[
                self.next();
                let left = self.parse();
                self.expect(&Token::Until);
                let right = self.parse();
                self.expect(&Token::RBracket);
                AstNode::AU(Box::new(left), Box::new(right))
            },
            Some(Token::EU) => {
                self.next(); // E[
                self.next();
                let left = self.parse();
                self.expect(&Token::Until);
                let right = self.parse();
                self.expect(&Token::RBracket);
                AstNode::EU(Box::new(left), Box::new(right))
            },
            Some(Token::LParen) => {
                self.next();
                let s = self.parse();
                self.expect(&Token::RParen);
                s
            }
            Some(Token::True) => { self.next(); AstNode::True }
            Some(Token::False) => { self.next(); AstNode::False }
            Some(Token::Identifier(name)) => { 
                let name = name.clone();
                self.next(); 
                AstNode::Id(name) 
            }
            _ => panic!("Unexpected token in primary expression: {:?}", self.peek())
        }
    }

    fn expect(&mut self, expected: &Token) {
        let token = self.next();
        if token.as_ref() != Some(expected) {
            panic!("Expected {:?}, but got {:?} at pos {:?}", expected, token, self.pos);
        }
    }
}