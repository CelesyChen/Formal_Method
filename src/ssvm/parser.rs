use std::collections::HashMap;
use std::io::copy;

use crate::ssvm::tokenizer::*;
use crate::ssvm::ds::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    prog: ProgramStats,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0, prog: ProgramStats::new() }
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

    pub fn parse(&mut self) {
        self.parse_program();
    }

    fn parse_program(&mut self) {
        self.parse_vad();
        self.parse_spec();
    }

    fn parse_vad(&mut self) {
        match self.peek() {
            Some(Token::Var) => { self.parse_var(); },
            Some(Token::Assign) => { self.parse_assign(); },
            Some(Token::Define) => { self.parse_define(); },
            None => { return; },
            _ => {
                panic!("Not available token {:?}", self.peek());
            }
        }

    }

    fn parse_var(&mut self) {
        self.match_token(Token::Var);
        self.parse_decl();
    }

    fn parse_decl(&mut self) {
        
        while let Some(Token::Id(name)) = self.peek() {
            let name = name.clone();
            self.next();
            self.expect(&Token::Colon);

            if self.match_token(Token::Int) {
                self.expect(&Token::DefineOp);
                let range = self.parse_num_range();
            
                self.prog
                    .vars
                    .val
                    .insert(name ,range);

            } else if self.match_token(Token::Bool) {
                if self.match_token(Token::True) {
                    self.prog
                        .vars
                        .val
                        .insert(name, VarRange { start: 1, end: None });
                } else if self.match_token(Token::False) {
                    self.prog
                        .vars
                        .val
                        .insert(name, VarRange { start: 0, end: None });
                } else {
                    panic!("Not available token {:?}", self.peek());
                }
            } else {
                panic!("Not available token {:?}", self.peek());
            }
            self.expect(&Token::Semicolon);
        }

    }

    fn parse_num_range(&mut self) -> VarRange {
        let l = match self.next() {
            Some(Token::Num(n)) => n,
            other => {
                panic!("Expected number at start of _num_range, got {:?}", other);
                0
            },
        };

        match self.peek() {
            Some(Token::DotDot) => {
                self.next();
                let r = match self.next() {
                    Some(Token::Num(n)) => n,
                    other => {
                        panic!("Expected number at second of _num_range, got {:?}", other);
                        0
                    },
                };

                if l > r {
                    panic!("Usage: l..r, where l <= r");
                }

                VarRange {
                    start: l,
                    end: Some(r),
                }
            },
            _ => {
                VarRange {
                    start: l,
                    end : None,
                }
            },
        }
    }

    fn parse_assign(&mut self) {
        self.match_token(Token::Assign);
        self.parse_ni();
    }

    fn parse_ni(&mut self) {
        loop {
            match self.peek() {
                Some(Token::Init) => {
                    self.parse_init();
                },
                Some(Token::Next) => {
                    self.parse_next();
                },
                _ => break,
            }
        }
    }

    fn parse_init(&mut self) {
        self.expect(&Token::Init);
        self.expect(&Token::LParen);
        
        let id = match self.next() {
            Some(Token::Id(name)) => name,
            other => {
                panic!("Expected number at start of _num_range, got {:?}", other);
            },
        };

        self.expect(&Token::RParen);
        self.expect(&Token::DefineOp);

        let mut value: u32 = 0;
        match self.peek() {
            Some(Token::True) => {
                value = 1;
            },
            Some(Token::False) => {},
            Some(Token::Num(n)) => {
                value = *n;
            },
            _ => {
                panic!("Not available token {:?}", self.peek());
            }
        }

        self.next();

        self.expect(&Token::Semicolon);

        self.prog
            .init
            .val
            .insert(id, value);

    }

    fn parse_next(&mut self) {
        self.expect(&Token::Init);
        self.expect(&Token::LParen);
        
        let id = match self.next() {
            Some(Token::Id(name)) => name,
            other => {
                panic!("Expected number at start of _num_range, got {:?}", other);
            },
        };

        self.expect(&Token::RParen);
        self.expect(&Token::DefineOp);
        self.expect(&Token::Case);
        self.parse_case_list(id);
        self.expect(&Token::Esae);
        self.expect(&Token::Semicolon);
    }

    fn parse_case_list(&mut self, id: String) {
        loop {
            match self.peek() {
                Some(Token::Id(name)) => {
                    
                },
                Some(Token::CaseDefault) => {

                },
                _ => break,
            }
        }
    }

    fn parse_define(&mut self) {
        
    }

    fn parse_def_list(&mut self) {
        
    }

    fn parse_spec(&mut self) {

    }
}