mod parser;
use std::fs;
mod bdd;
mod ctl;
use ctl::parse_ast;
use crate::bdd::BddManager;
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
  let unparse_file = fs::read_to_string("tests/test.ssmv").expect("cannot open file");
  let pairs = SSVMParser::parse(Rule::program, &unparse_file).expect("parse failed");

  // print_pairs(pairs.clone(), 0);

  let ast = build_ast(pairs.peek().unwrap());

  // println!("{:#?}", &ast);

  let a = table::to_symbol_table(&ast);
  // for item in a {
  //   println!("{:#?}", item);
  // }

  for st in a {
    let order = st.suggest_variable_order();
    let bv = st.to_bv_with_order(order);
    let mut manager = BddManager::new(bv);
    // for (name, var) in st.contain.iter() {
    //   // if name != "signal" { continue; }
    //   let bdd = manager.encode_var_next(name, var);
    //   manager.print_bdd(bdd, 0);
    //   // println!("remove:");
    //   // let bdd = manager.rename_vars(bdd, false);
    //   // manager.print_bdd(bdd, 0);
    // }
    let trans = manager.encode_whole_transition(&st);
    // manager.print_bdd(trans, 0);
    for ctl in st.specs {
      // println!("{:#?}", &ctl);
      let a = manager.ctl_to_bdd(&ctl, trans);
      // manager.print_bdd(a, 0);
      let _ = manager.export_dot(a, "tests/bdd.dot");
    }
    
  }
}
