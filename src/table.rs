use crate::parser::ssvmparser::*;
use core::panic;
use std::{collections::{HashMap, HashSet, VecDeque}, hash::{Hash}};

use crate::ctl::{parse_ast, ast::CtlAst};

#[derive(Debug)]
pub struct SymbolTable {
  pub contain: HashMap<String, Variable>,
  pub specs: Vec<CtlAst>
}

#[derive(Debug)]
pub struct Variable { 
  pub domain: Range,
  pub init: u32,
  pub next: HashMap<Val, Expr>,
  pub mapping: Option<HashMap<String, u32>>, // only for enum
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Val {
  Num(u32),
  Id(String),
}


#[derive(Debug)]
pub struct Range {
  pub start: u32,
  pub end: u32,
} 

pub fn to_symbol_table ( ast: &AstNode ) -> Vec<SymbolTable> {

  match ast {
    AstNode::Program(vec) => {
      vec.iter().map(per_module).collect()
    }
    _ => unreachable!()
  }

}

fn per_module (
  ast: &AstNode
) -> SymbolTable {
  let mut a = SymbolTable { contain: HashMap::new(), specs: Vec::new() };
  match ast {
    AstNode::ModuleDecl { name: _, body } => {
      for temp in body {
        match temp {
          AstNode::VarDecl { id, ty } => {
            let range;
            let map;
            match ty {
              SVMType::Bool => {
                range = Range { start: 0, end: 1 };
                map = None;
              }
    
              SVMType::Enum(strs) => {
                range = Range { start: 0, end: strs.len() as u32 - 1 };
                let mut hashmap = HashMap::new();
                for (idx, str) in strs.into_iter().enumerate() {
                  hashmap.insert(str.clone(), idx as u32);
                }
                map = Some(hashmap);
              }
    
              SVMType::Int(l, r) => {
                range = Range { start: *l, end: *r };
                map = None;
              }
            }
            
            let var = Variable {
              domain: range, 
              init: 0, 
              next: HashMap::new(),
              mapping: map
            };
            a.contain.insert(id.clone(), var);
            
          }
          // 我还没想好这个怎么做，并且这次也用不上，所以暂不完成
          AstNode::DefineDecl(..) => {
            unimplemented!()
          }
    
          AstNode::Assign( name, expr ) => {
            let var = match a.contain.get_mut(name) {
              Some(v) => v,
              None => panic!("Initialization on none defined variable.") 
            };
    
            match expr {
              AssignExpr::Case(items) => {
                for item in items {
                  for atom in &item.result {
                    var.next.insert(proc_atom(atom, &var.mapping), item.expr.clone());
                  }
                }
              }
    
              AssignExpr::Single(atoms) => {
                for atom in atoms {
                  var.next.insert(proc_atom(atom, &var.mapping), Expr::True(true));
                }
              }
            }
          }
    
          AstNode::Init( name, val ) => {
            let var = match a.contain.get_mut(name) {
              Some(v) => v,
              None => panic!("Initialization on none defined variable.") 
            };

            var.init = match proc_atom(val, &var.mapping) {
              Val::Num(n) => n,
              _ => panic!("Cannot initialize a variable with another variable")
            }
    
          }

          AstNode::Spec(spec) => {
            a.specs.push(parse_ast(spec));
          }

          _ => unreachable!()
        }
      }

    }
    _ => unreachable!()
  }
  a
}

fn proc_atom (
  atom: &Atom,
  map: &Option<HashMap<String, u32>>,
) -> Val {

  match atom {
    Atom::Bool(b) => {
      Val::Num(*b as u32)
    }

    Atom::Id(id) => {
      match map {
        Some(m) => {
          match m.get(id) {
            Some(i) => {
              Val::Num(*i as u32)
            },
            None => panic!("Enum not exist"),
          }
          
        }
        // id
        None => {
          Val::Id(id.clone())
        }
      }
    }

    Atom::Num(v) => {
      Val::Num(*v as u32)
    }
  }
}

impl SymbolTable {
  pub fn suggest_variable_order(&self) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    let mut freq: HashMap<String, usize> = HashMap::new();

    for (name, var) in &self.contain {
      *freq.entry(name.clone()).or_insert(0) += var.next.len();

      for (val, cond) in &var.next {
        collect_vars_from_expr(cond, &mut freq);

        if let Val::Id(other_name) = val {
          *freq.entry(other_name.clone()).or_insert(0) += 1;
        }
      }

      *freq.entry(name.clone()).or_insert(0) += 1;
    }

    let mut vars: Vec<_> = freq.into_iter().collect();
    vars.sort_by(|a, b| b.1.cmp(&a.1)); 

    vars.into_iter().map(|(v, _)| v).collect()
  }
}


fn collect_vars_from_expr(expr: &Expr, freq: &mut HashMap<String, usize>) {
  match expr {
    Expr::Eq(var, atom) | Expr::Ne(var, atom) => {
      *freq.entry(var.clone()).or_insert(0) += 1;
      if let Atom::Id(s) = atom {
        *freq.entry(s.clone()).or_insert(0) += 1;
      }
    }
    Expr::And(l, r) | Expr::Or(l, r) => {
      collect_vars_from_expr(l, freq);
      collect_vars_from_expr(r, freq);
    }
    Expr::True(_) => {}
  }
}
