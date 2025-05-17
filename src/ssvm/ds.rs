use std::collections::HashMap;
use crate::ctl::ast::{self, AstNode};


#[derive(Debug)]
pub struct ProgramStats {
    vars: Var,
    init: Init,
    trans: Trans,
    defs: Vec<Def>,
    specs: Vec<Spec>
}

#[derive(Debug)]
struct Var {
    val: HashMap< String, VarRange>
}

#[derive(Debug)]
pub struct VarRange {
    start: u32, 
    end: Option<u32> // 若非范围值，这个为空
}

#[derive(Debug)]
struct Init {
    val: HashMap<String, u32>
}

#[derive(Debug)]
struct Trans {
    trans_table: HashMap<String, HashMap<u32, Vec<u32>>>
}

#[derive(Debug)]
struct Def {
    def_list: HashMap<String, In_list>
}

#[derive(Debug)]
struct In_list {
    var: String,
    state: Vec<u32>
}

#[derive(Debug)]
struct Spec {
    speclist: Vec<Box<AstNode>>
}

// graphlib
pub struct StateGraph {

}