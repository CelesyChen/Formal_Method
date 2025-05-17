mod ctl;
use ctl::tokenizer::{tokenize as ctl_tokenize};
// use ctl::parser as ctl_parser;
// use ctl::ast as ctl_ast;

use std::io::Read;
mod ssvm;
use ssvm::tokenizer::{tokenize as ssvm_tokenize};

mod robdd;


fn main() {
let mut file = std::fs::File::open("../lab2/first.smv").unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();
   // print!("{}", contents);
   let vec = ssvm_tokenize(&contents);
   print!("{:?}",vec);

}