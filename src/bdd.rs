use std::{collections::HashMap, fmt::Debug, hash::Hash};
use petgraph::graph::Node;

use crate::table::{Range, SymbolTable, Variable, Val};
use crate::parser::ssvmparser::{Expr as ssvmExpr, Atom as ssvmAtom};
use crate::ctl::ast::{CtlAst, Atom as CtlAtom};

// 定义 ROBDD 节点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] 
pub struct NodeId(usize); 

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BddNode {
  Terminal(bool),
  NonTerminal {
      variable: usize, 
      low: NodeId,
      high: NodeId,
  },
}

// ROBDD 管理器
#[derive(Debug)]
pub struct BddManager {
  unique_table: HashMap<(usize, NodeId, NodeId), NodeId>,
  nodes: Vec<BddNode>,
  computed_table: HashMap<(OpType, NodeId, NodeId), NodeId>, 
  bv: BoolVariables,
  restrict_cache: HashMap<(NodeId, usize, bool), NodeId>
}

impl BddManager {
  pub fn new(bv: BoolVariables) -> Self {
    let mut manager = BddManager {
      unique_table: HashMap::new(),
      nodes: Vec::new(),
      computed_table: HashMap::new(),
      bv,
      restrict_cache: HashMap::new()
    };

    manager.add_node(BddNode::Terminal(true));
    manager.add_node(BddNode::Terminal(false));

    manager
  }

  fn add_node(&mut self, node: BddNode) -> NodeId {
    let id = self.nodes.len();
    self.nodes.push(node);
    NodeId(id)
  }
  
  pub fn get_true_node(&self) -> NodeId {
    NodeId(0) 
  }

  pub fn get_false_node(&self) -> NodeId {
    NodeId(1)
  }

  pub fn get_or_create_node(&mut self, variable: usize, low: NodeId, high: NodeId) -> NodeId {

    if low == high {
      return low;
    }

    let key = (variable, low, high);
    if let Some(&node_id) = self.unique_table.get(&key) {
      return node_id; 
    }

    // 节点不存在
    let new_node = BddNode::NonTerminal { variable, low, high };
    let new_node_id = self.add_node(new_node);
    self.unique_table.insert(key, new_node_id);
    new_node_id
  }

  pub fn get_node(&self, id: NodeId) -> &BddNode {
    &self.nodes[id.0]
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpType {
  Not,
  And,
  Or,
  Imply,
  Eq,
}

#[derive(Debug)]
pub struct BoolVariables {
  pub var_to_idx: HashMap<String, Range>,
  pub idx_to_var: Vec<String>,
}

impl SymbolTable {
  pub fn to_bv_with_order(&self, order: Vec<String>) -> BoolVariables {
    let mut next = 0usize;
    let mut bv = BoolVariables {
      var_to_idx: HashMap::new(),
      idx_to_var: Vec::new(),
    };

    for name in order {
      let name_prime = name.clone() + "'";
      let var = self.contain.get(&name).expect("var not found in SymbolTable");
      let len = ((var.domain.end - var.domain.start + 1) as f32).log2().ceil() as usize;

      for _ in 0..len {
        bv.idx_to_var.push(name.clone());
        bv.idx_to_var.push(name_prime.clone()); 
      }

      bv.var_to_idx.insert(name.clone(), Range { start: next as u32, end: (next + 2 * len - 1) as u32 });
      next += 2 * len;
    }

    bv
  }
}


impl BddManager {
  pub fn mk_var(&mut self, var: &str, is_prime: bool) -> Vec<usize> {
    let range = self.bv.var_to_idx.get(var).expect("Variable not found in BoolVariables");

    let mut result = vec![];
    for i in range.start..=range.end {
      let idx = i as usize;
      let is_prime_slot = idx % 2 == 1;

      if is_prime == is_prime_slot {
        result.push(idx);
      }
    }

    result
  }

  pub fn encode_val_to_bdd(&mut self, var: &str, val: u32, is_prime: bool) -> NodeId {
    let bits = self.mk_var(var, is_prime);

    let result = self.encode_val_rec(
        &bits,
        0,
        0,
        val,
    );
    // self.print_bdd(result, 0);
    result
  }
  fn encode_val_rec(
    &mut self,
    bits_indices: &[usize], 
    current_bit_idx: usize, 
    current_path_val: u32,  
    target_val: u32,       
  ) -> NodeId {
    if current_bit_idx == bits_indices.len() {
      if current_path_val == target_val {
        return self.get_true_node();
      } else {
        return self.get_false_node();
      }
    }

    let var_index = bits_indices[current_bit_idx]; 

    let low_child = self.encode_val_rec(
        bits_indices,
        current_bit_idx + 1, 
        current_path_val,  
        target_val,
    );
    let high_child = self.encode_val_rec(
        bits_indices,
        current_bit_idx + 1, 
        current_path_val | (1 << current_bit_idx), 
        target_val,
    );

    self.get_or_create_node(var_index, low_child, high_child)
  }

  pub fn expr_to_bdd(&mut self, expr: &ssvmExpr) -> NodeId {
    match expr {
      ssvmExpr::True(b) => {
        if *b {
          self.get_true_node()
        } else {
          self.get_false_node()
        }
      }

      ssvmExpr::Eq(name, atom) => {
        self.encode_eq(name, atom, false) 
      }

      ssvmExpr::Ne(name, atom) => {
        let eq_bdd = self.encode_eq(name, atom, false);
        self.not(eq_bdd)
      }

      ssvmExpr::And(e1, e2) => {
        let bdd1 = self.expr_to_bdd(e1);
        // 剪枝：false && _ = false
        if bdd1 == self.get_false_node() {
          return bdd1;
        }

        let bdd2 = self.expr_to_bdd(e2);
        // 剪枝：_ && false = false
        if bdd2 == self.get_false_node() {
          return bdd2;
        }

        // 剪枝：true && b = b
        if bdd1 == self.get_true_node() {
          return bdd2;
        } else if bdd2 == self.get_true_node() {
          return bdd1;
        }

        self.apply(OpType::And, bdd1, bdd2)
      }

      ssvmExpr::Or(e1, e2) => {
        let bdd1 = self.expr_to_bdd(e1);
        if bdd1 == self.get_true_node() {
          return bdd1;
        }

        let bdd2 = self.expr_to_bdd(e2);
        if bdd2 == self.get_true_node() {
          return bdd2;
        }

        if bdd1 == self.get_false_node() {
          return bdd2;
        } else if bdd2 == self.get_false_node() {
          return bdd1;
        }

        self.apply(OpType::Or, bdd1, bdd2)
      }
    }
  }

  pub fn encode_var_next(&mut self, var_name: &str, var: &Variable) -> NodeId {
    let mut res = self.get_false_node();

    for (val, cond_expr) in &var.next {
      // 1. 条件的布尔表达式
      let cond_bdd = self.expr_to_bdd(cond_expr);
      // 2. 目标值的布尔编码
      let val_bdd = match val {
        Val::Num(c) => {
          self.encode_val_to_bdd(var_name, *c, true)
        }
        Val::Id(other_var) => {
          self.encode_eq_var(var_name, true, other_var, false)
        }
      };

      // 3. 由条件守护的更新项
      let guarded = self.apply(OpType::And, cond_bdd, val_bdd);

      // 4. 累积所有项的并集
      res = self.apply(OpType::Or, res, guarded);
    }
    res
  }

  fn encode_eq(&mut self, name: &str, atom: &ssvmAtom, is_prime: bool) -> NodeId {
    match atom {
      ssvmAtom::Bool(b) => {
        let var_bdd = self.encode_val_to_bdd(name, if *b { 1 } else { 0 }, is_prime);
        var_bdd
      }
      ssvmAtom::Num(n) => {
        self.encode_val_to_bdd(name, *n, is_prime)
      }
      ssvmAtom::Id(other) => {
        self.encode_eq_var(name, is_prime, other, is_prime)
      }
    }
  }

  fn encode_eq_var(
    &mut self,
    left: &str,
    is_left_prime: bool,
    right: &str,
    is_right_prime: bool
  ) -> NodeId {
    let left_bits = self.mk_var(left, is_left_prime);
    let right_bits = self.mk_var(right, is_right_prime);
    assert_eq!(left_bits.len(), right_bits.len());

    let result = self.encode_eq_var_rec(
        &left_bits,
        &right_bits,
        0,
    );
    result
  }
  fn encode_eq_var_rec(
    &mut self,
    left_bits: &[usize], 
    right_bits: &[usize], 
    idx: usize
  ) -> NodeId {
    let a = self.get_or_create_node(left_bits[idx], self.get_false_node(), self.get_true_node());
    let b = self.get_or_create_node(right_bits[idx], self.get_false_node(), self.get_true_node());
    let a_and_b = self.apply(OpType::And, a, b);
    let nota = self.not(a);
    let notb = self.not(b);
    let nota_and_notb = self.apply(OpType::And, nota, notb);
    let eq = self.apply(OpType::Or, a_and_b, nota_and_notb);

    if idx == left_bits.len() - 1 {
      return eq;
    }

    let child = self.encode_eq_var_rec(
      left_bits,
      right_bits,
      idx + 1,
    );

    self.apply(OpType::And, eq, child )
  }

  pub fn encode_whole_transition(&mut self, symbol_table: &SymbolTable) -> NodeId {
    let mut res = self.get_true_node(); 
    for (name, var) in symbol_table.contain.iter() {
      let bdd = self.encode_var_next(name, var);
      res = self.apply(OpType::And, res, bdd);
    }
    res
  }

  fn not(&mut self, a: NodeId) -> NodeId {
    self.apply(OpType::Imply, a, self.get_false_node())
  }

  fn apply(&mut self, op: OpType, a: NodeId, b: NodeId) -> NodeId {
    // Early reduction rules
    let t = self.get_true_node();
    let f = self.get_false_node();

    match op {
      OpType::And => {
        if a == f || b == f {
          return f;
        } else if a == t {
          return b;
        } else if b == t {
          return a;
        }
      }
      OpType::Or => {
        if a == t || b == t {
          return t;
        } else if a == f {
          return b;
        } else if b == f {
          return a;
        }
      }
      OpType::Imply => {
        if a == f || b == t {
          return t;
        } else if a == t {
          return b;
        }
      }
      _ => {}
    }

    let key = (op, a, b);
    if let Some(&cached) = self.computed_table.get(&key) {
      return cached;
    }

    let a_node = self.get_node(a).clone();
    let b_node = self.get_node(b).clone();

    let res = match (a_node, b_node) {
      (BddNode::Terminal(x), BddNode::Terminal(y)) => {
        let result = match op {
          OpType::And => x && y,
          OpType::Or => x || y,
          OpType::Imply => !x || y,
          OpType::Eq => x == y,
          _ => unreachable!(),
        };
        if result {
          self.get_true_node()
        } else {
          self.get_false_node()
        }
      }
      (BddNode::NonTerminal { variable: v1, low: l1, high: h1 },
      BddNode::NonTerminal { variable: v2, low: l2, high: h2 }) => {
        let var = v1.min(v2);
        let (l1p, h1p) = if v1 == var { (l1, h1) } else { (a, a) };
        let (l2p, h2p) = if v2 == var { (l2, h2) } else { (b, b) };

        let low = self.apply(op, l1p, l2p);
        let high = self.apply(op, h1p, h2p);
        self.get_or_create_node(var, low, high)
      }
      (BddNode::Terminal(_), BddNode::NonTerminal{ variable, low, high }) => {
        let low = self.apply(op, a, low);
        let high = self.apply(op, a, high);
        self.get_or_create_node(variable, low, high)
      }
      (BddNode::NonTerminal{ variable, low, high }, BddNode::Terminal(_)) => {
        let low = self.apply(op, low, b);
        let high = self.apply(op, high, b);
        let temp = self.get_or_create_node(variable, low, high);
        // self.print_bdd(temp, 0);
        temp
      }
    };

    self.computed_table.insert(key, res);
    res
  }


}


// CTL部分
impl BddManager {
  pub fn ctl_to_bdd(&mut self, ast: &CtlAst, trans: NodeId) -> NodeId {
    // println!("trans:{}", trans.0);
    match ast {
      CtlAst::True => self.get_true_node(),
      CtlAst::False => self.get_false_node(),
      CtlAst::Expr(name, val) => {
        match val {
          CtlAtom::Num(n) => {
            self.encode_val_to_bdd(name, *n, false)
          }
          CtlAtom::Id(other) => {
            self.encode_eq_var(name, false, other, false)
          }
        }
      }

      CtlAst::Not(inner) => {
        let b = self.ctl_to_bdd(inner, trans);
        self.not(b)
      }
      CtlAst::And(l, r) => {
        let l_bdd = self.ctl_to_bdd(l, trans);
        let r_bdd = self.ctl_to_bdd(r, trans);
        self.apply(OpType::And, l_bdd, r_bdd)
      }
      CtlAst::Or(l, r) => {
        let l_bdd = self.ctl_to_bdd(l, trans);
        let r_bdd = self.ctl_to_bdd(r, trans);
        self.apply(OpType::Or, l_bdd, r_bdd)
      }
      CtlAst::Implies(l, r) => {
        let l_bdd = self.ctl_to_bdd(l, trans);
        let r_bdd = self.ctl_to_bdd(r, trans);
        self.apply(OpType::Imply, l_bdd, r_bdd)
      }

      CtlAst::EU(l, r) => {
        let phi_l = self.ctl_to_bdd(l, trans);
        let phi_r = self.ctl_to_bdd(r, trans);
        self.eval_ctl_eu(phi_l, phi_r, trans)
      }

      // 其他时序逻辑以后逐步加
      _ => todo!("CTL operator not implemented yet: {:?}", ast)
    }
  }

  pub fn eval_ctl_eu(&mut self, phi_l: NodeId, phi_r: NodeId, trans: NodeId) -> NodeId {
    // μZ . phi2 ∨ (phi1 ∧ EX(Z))
    let mut z = phi_r;  // Initial guess: Z₀ = ϕ₂
    let mut iter = 0;
    loop {
      println!("{}", iter);
      iter += 1;
      // 1. Compute pre_image: EX(z)
      // Step 1: rename z into primed variables
      let z_prime = self.rename_vars(z, true);

      // Step 2: pre_image = ∃ x'.( trans(x, x') ∧ z'(x') )
      let pre_image = self.apply(OpType::And,trans, z_prime);
      let pre_states = self.exist_quantify(pre_image);

      // 2. Compute new Z = phi2 ∨ (phi1 ∧ pre_states)
      let phi1_and_pre = self.apply(OpType::And, phi_l, pre_states);
      let new_z = self.apply(OpType::Or, phi_r, phi1_and_pre);

      println!("new:{} old:{}", new_z.0, z.0);
      if new_z == z {
        break;
      }

      z = new_z;
    }

    z
  }

  // 将 BDD 中所有 primed 变量（如 x'）替换成 unprimed（如 x）
  pub fn rename_vars(&mut self, node: NodeId, to_prime: bool) -> NodeId {

    let res = match self.get_node(node).clone() {
      BddNode::Terminal(_) => node,
      BddNode::NonTerminal { variable, low, high } => {
        // 偶数是 unprimed，奇数是 primed
        let new_var = if to_prime {
          if variable % 2 == 0 { variable + 1 } else { variable }
        } else {
          if variable % 2 == 1 { variable - 1 } else { variable }
        };

        let new_low = self.rename_vars(low, to_prime);
        let new_high = self.rename_vars(high, to_prime);
        self.get_or_create_node(new_var, new_low, new_high)
      }
    };

    res
  }

  fn exist_quantify(&mut self, bdd: NodeId) -> NodeId {
    let mut result = bdd;
    let vars = self.next_vars();
    for v in vars {
      let low = self.restrict(result, v, false);
      let high = self.restrict(result, v, true);
      result = self.apply(OpType::Or, low, high);
    }
    result
  }

  fn next_vars(&self) -> Vec<usize> {
    let mut vars = vec![];
    for (_var, range) in &self.bv.var_to_idx {
      for i in range.start as usize..=range.end as usize {
        if i % 2 == 1 {
          vars.push(i);
        }
      }
    }
    vars
  }

  // 对BDD中的变量 var 赋值为 value，返回约束后的BDD
  fn restrict(&mut self, node: NodeId, var: usize, value: bool) -> NodeId {
    if let Some(&cached) = self.restrict_cache.get(&(node, var, value)) {
      return cached;
    }

    let node_data = self.get_node(node).clone();
    let res = match node_data {
      BddNode::Terminal(_) => node,
      BddNode::NonTerminal { variable, low, high } => {
        if variable == var {
          // 变量正好是要赋值的变量，直接返回对应分支
          if value {
            self.restrict(high, var, value)
          } else {
            self.restrict(low, var, value)
          }
        } else {
          // 否则递归子节点
          let new_low = self.restrict(low, var, value);
          let new_high = self.restrict(high, var, value);
          self.get_or_create_node(variable, new_low, new_high)
        }
      }
    };

    self.restrict_cache.insert((node, var, value), res);
    res
  }

}

// Debugging
impl BddManager {
  pub fn print_bdd(&self, node: NodeId, indent: usize) {
    let padding = "  ".repeat(indent);
    match self.get_node(node) {
      BddNode::Terminal(b) => {
        println!("{}Terminal({})", padding, b);
      }
      BddNode::NonTerminal { variable, low, high } => {
        let name = self.bv.idx_to_var[*variable].clone();
        println!("{}Var {}_{}:", padding, &name , ( *variable as u32 - self.bv.var_to_idx.get(&name).unwrap().start ) / 2 );
        println!("{}  Low:", padding);
        self.print_bdd(*low, indent + 2);
        println!("{}  High:", padding);
        self.print_bdd(*high, indent + 2);
      }
    }
  }
}