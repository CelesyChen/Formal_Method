mod parser;
use std::fs;
mod ctl;
use ctl::{parse_ast};
mod table;

use crate::parser::ssvmparser::*;

use pest::Parser;

// for debug
fn print_pairs(pairs: pest::iterators::Pairs<Rule>, depth: usize) {
    for pair in pairs {
        println!(
            "{}{:?} => {}",
            "  ".repeat(depth),
            pair.as_rule(),
            pair.as_str()
        );
        print_pairs(pair.into_inner(), depth + 1);
    }
}

fn collect_spec_values(ast: &AstNode, specs: &mut Vec<String>) {
  match ast {
    AstNode::Program(nodes) => {
      for node in nodes {
        collect_spec_values(node, specs);
      }
    }
    AstNode::ModuleDecl { name: _, body } => {
      for node in body {
        collect_spec_values(node, specs);
      }
    }
    AstNode::Spec(spec_str) => {
      specs.push(spec_str.clone());
    }
    _ => {} 
  }
}


fn main() {
  let unparse_file = fs::read_to_string("tests/test.smv")
    .expect("cannot open file");
  let pairs = SSVMParser::parse(Rule::program, &unparse_file).expect("parse failed");

  // print_pairs(pairs, 0);

  let ast = build_ast(pairs.peek().unwrap());

  // println!("{:#?}", &ast);

  let a = table::to_symbol_table(&ast);
  for item in a {
    println!("{:#?}",item);
  }
  
}