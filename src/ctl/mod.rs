use ast::CtlAst;
use parser::CtlParser;
use tokenizer::tokenize;

pub mod tokenizer;
pub mod parser;
pub mod ast;

pub fn parse_ast(input: &str) -> CtlAst {
  let tokens = tokenize(&input);
  let mut parser = CtlParser::new(tokens);
  parser.parse().norm_and_opt()
}