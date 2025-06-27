use std::vec;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"] // relative to src
pub struct SSVMParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    ModuleDecl { name: String, body: Vec<AstNode> },
    VarDecl { id: String, ty: SVMType },
    DefineDecl(String, String, Vec<Atom>), // id := id in {...}
    Assign(String, AssignExpr),            // next(x) := ...
    Init(String, Atom),
    Spec(String),
}

#[derive(Debug, Clone)]
pub enum SVMType {
    Bool,
    Int(u32, u32), // from to
    Enum(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum AssignExpr {
    Case(Vec<CaseItem>),
    Single(Vec<Atom>),
}

#[derive(Debug, Clone)]
pub struct CaseItem {
    pub expr: Expr,
    pub result: Vec<Atom>,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Bool(bool),
    Id(String),
    Num(u32),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Eq(String, Atom),
    Ne(String, Atom),
    True(bool),
}

use pest;
use pest::iterators::Pair;

pub fn build_ast(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        // program = { SOI ~ (module_decl)* ~ EOI }
        Rule::program => {
            let nodes = pair
                .into_inner()
                .filter(|p| p.as_rule() == Rule::module_decl)
                .map(build_ast)
                .collect();
            AstNode::Program(nodes)
        }

        // module_decl = { "MODULE" ~ identifier ~ module_body }
        Rule::module_decl => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();

            let body = build_module_body(inner.next().unwrap());

            AstNode::ModuleDecl { name, body }
        }

        // var_list = { identifier ~ ":" ~ type ~ ";" }
        Rule::var_list => {
            let mut inner = pair.into_inner();
            let id = inner.next().unwrap().as_str().to_string();
            let ty = build_type(inner.next().unwrap());
            AstNode::VarDecl { id, ty }
        }

        // define_list = { identifier ~ ":=" ~ identifier ~ "in" ~ "{" ~ atom ~ ("," ~ atom)* ~ "}" ~ ";" }
        Rule::define_list => {
            let mut inner = pair.into_inner();
            let id = inner.next().unwrap().as_str().to_string();
            let ref_id = inner.next().unwrap().as_str().to_string();
            let atoms = inner.next().unwrap().into_inner().map(build_atom).collect();
            AstNode::DefineDecl(id, ref_id, atoms)
        }

        // assign_list = { "next" ~ "(" ~ identifier ~ ")" ~ ":=" ~ (case_assign | single_assign) ~ ";" }
        Rule::assign_list => {
            let mut inner = pair.into_inner();
            let var = inner.next().unwrap().as_str().to_string();
            let assign_expr = inner.next().unwrap();
            let expr = match assign_expr.as_rule() {
                Rule::case_assign => {
                    let items = assign_expr.into_inner().map(build_case_item).collect();
                    AssignExpr::Case(items)
                }
                Rule::single_assign => {
                    let atom = assign_expr.into_inner().next().unwrap().into_inner().map(build_atom).collect();
                    AssignExpr::Single(atom)
                }
                _ => unreachable!(),
            };
            AstNode::Assign(var, expr)
        }

        Rule::init_list => {
            let mut inner = pair.into_inner();
            let var = inner.next().unwrap().as_str().to_string();
            let value = build_atom(inner.next().unwrap());
            AstNode::Init(var, value)
        }

        Rule::spec_list => {
            let ctl_str = pair
                .into_inner()
                .next()
                .unwrap()
                .as_str()
                .trim()
                .to_string();
            AstNode::Spec(ctl_str)
        }

        _ => unreachable!("{:?}", pair.as_rule()),
    }
}

fn build_module_body(pair: Pair<Rule>) -> Vec<AstNode> {
    pair.into_inner().map(build_ast).collect()
}

fn build_type(pair: Pair<Rule>) -> SVMType {
    let mut inner = pair.into_inner();
    let next = inner.next().unwrap();

    match next.as_rule() {
        Rule::Bool => SVMType::Bool,
        Rule::Int => {
            let l = inner.next().unwrap().as_str().parse().unwrap();
            let r = inner.next().unwrap().as_str().parse().unwrap();
            SVMType::Int(l, r)
        }
        Rule::Enum => {
            let enums = inner
                .filter(|p| p.as_rule() == Rule::identifier)
                .map(|p| p.as_str().to_string())
                .collect();

            SVMType::Enum(enums)
        }
        other => unreachable!("{:?}", other),
    }
}

fn build_case_item(pair: Pair<Rule>) -> CaseItem {
    let mut inner = pair.into_inner();
    let current_pair = inner.next().unwrap(); 
    let expr = match current_pair.as_rule() {
        Rule::TRUE => Expr::True(true),
        Rule::expr => build_expr(current_pair), 
        _ => unreachable!("Should be an Expression or TRUE")
    };
    let result = inner.next().unwrap().into_inner().map(build_atom).collect();
    CaseItem { expr, result }
}

fn build_atom(pair: Pair<Rule>) -> Atom {
    let t = pair.into_inner().next().unwrap();
    match t.as_rule() {
        Rule::TRUE => Atom::Bool(true),
        Rule::FALSE => Atom::Bool(false),
        Rule::identifier => Atom::Id(t.as_str().to_string()),
        Rule::number => Atom::Num(t.as_str().parse().unwrap()),
        _ => unreachable!("{:?}", t.as_rule()),
    }
}

fn build_expr(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr | Rule::or_expr => {
            let mut inner = pair.into_inner();
            let mut lhs = build_expr(inner.next().unwrap());

            while let Some(op_pair) = inner.next() {
                let rhs = build_expr(inner.next().unwrap());

                lhs = match op_pair.as_rule() {
                    Rule::Or => Expr::Or(Box::new(lhs), Box::new(rhs)),
                    Rule::And => Expr::And(Box::new(lhs), Box::new(rhs)),
                    other => unreachable!("Unknown operator: {:?}", other),
                };
            }

            lhs
        }

        Rule::eq_expr => {
            let mut inner = pair.into_inner();
            let cur = inner.next().unwrap();
            let lhs = match cur.as_rule() {
                Rule::identifier => cur.as_str().to_string(),
                _ => unreachable!("")
            };

            let op = match inner.next().unwrap().as_rule() {
                Rule::Eq => true,
                Rule::Neq => false,
                _ => unreachable!("")
            };

            let rhs = build_atom(inner.next().unwrap());
            
            if op {
                Expr::Eq(lhs, rhs)
            } else {
                Expr::Ne(lhs, rhs)
            }
        }

        _ => unreachable!("Unexpected rule in build_expr: {:?}", pair.as_rule()),
    }
}
