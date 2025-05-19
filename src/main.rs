mod parser;
use std::fs;

use crate::parser::ssvmparser::*; // 替换为你的 pest 模块名

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

fn main() {
  let unparse_file = fs::read_to_string("tests/test.smv")
    .expect("cannot open file");
  let pairs = SSVMParser::parse(Rule::program, &unparse_file).expect("parse failed");

  // print_pairs(pairs, 0);

  let ast = build_ast(pairs.peek().unwrap());

  print!("{:#?}", ast);
}