use std::collections::HashMap;
use crate::ctl::ast::AstNode;


#[derive(Debug)]
pub struct ProgramStats {
    pub vars: Var,
    pub init: Init,
    pub trans: Trans,
    pub defs: Vec<Def>,
    pub specs: Vec<Spec>,
}

impl ProgramStats {
    pub fn new() -> Self {
        Self {
            vars: Var::new(),
            init: Init::new(),
            trans: Trans::new(),
            defs: vec![],
            specs: vec![],
        }
    }

}

#[derive(Debug)]
pub struct Var {
    pub val: HashMap<String, VarRange>,
}

impl Var {
    fn new() -> Self {
        Self {
            val: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct VarRange {
    pub start: u32,
    pub end: Option<u32>, // 若非范围值，这个为空
}

#[derive(Debug)]
pub struct Init {
    pub val: HashMap<String, u32>,
}

impl Init {
    fn new() -> Self {
        Self {
            val: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Trans {
    pub trans_table: HashMap<String, HashMap<u32, Vec<u32>>>,
}

impl Trans {
    fn new() -> Self {
        Self {
            trans_table: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Def {
    pub def_list: HashMap<String, InList>,
}

impl Def {
    fn new() -> Self {
        Self {
            def_list: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct InList {
    pub var: String,
    pub state: Vec<u32>,
}

impl InList {
    fn new(var: String) -> Self {
        Self {
            var,
            state: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Spec {
    pub speclist: Vec<Box<AstNode>>,
}

impl Spec {
    fn new() -> Self {
        Self {
            speclist: vec![],
        }
    }
}

// graphlib
pub struct StateGraph {

}