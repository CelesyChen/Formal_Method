use core::panic;

#[derive(Debug, Clone)]
pub enum AstNode {
    And(Box<AstNode>, Box<AstNode>),
    Or(Box<AstNode>, Box<AstNode>),
    Implies(Box<AstNode>, Box<AstNode>),
    Not(Box<AstNode>),
    AG(Box<AstNode>),
    EG(Box<AstNode>),
    AX(Box<AstNode>),
    EX(Box<AstNode>),
    AF(Box<AstNode>),
    EF(Box<AstNode>),
    AU(Box<AstNode>, Box<AstNode>),
    EU(Box<AstNode>, Box<AstNode>),
    Id(String),
    True,
    False,
}

impl AstNode {
    pub fn norm_and_opt(self) -> AstNode {
        self.normalize().optimize()
    }

    fn normalize(self) -> AstNode {
        match self {
            AstNode::And(lhs, rhs) => {
                AstNode::And(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            AstNode::Or(lhs, rhs) => {
                AstNode::Or(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            AstNode::Implies(lhs, rhs) => {
                AstNode::Implies(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            AstNode::Not(inner) => {
                AstNode::Not(Box::new(inner.normalize()))
            }
            AstNode::AG(inner) => {
                // AG x = not EF not x = not E [ T U not x]
                AstNode::Not(
                    Box::new(AstNode::EU(
                        Box::new(AstNode::True), 
                        Box::new(AstNode::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            AstNode::EG(inner) => {
                AstNode::EG(Box::new(inner.normalize()))
            }
            AstNode::AX(inner) => {
                // AX x = not EX not x
                AstNode::Not(
                    Box::new(AstNode::EX(
                        Box::new(AstNode::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            AstNode::EX(inner) => {
                AstNode::EX(Box::new(inner.normalize()))
            }
            AstNode::AF(inner) => {
                // AF x = not EG not x
                AstNode::Not(
                    Box::new(AstNode::EG(
                        Box::new(AstNode::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            AstNode::EF(inner) => {
                // EF x = E[T U x]
                AstNode::EU(Box::new(AstNode::True), 
                    Box::new(inner.normalize())
                )
            }
            AstNode::AU(left, right) => {
                // A[φ U ψ]  -->  ¬(E[¬ψ U (¬φ ∧ ¬ψ)] ∨ EG¬ψ)
                let not_right = AstNode::Not(Box::new(right.normalize()));
                let not_left = AstNode::Not(Box::new(left.normalize()));
                let conj = AstNode::And(Box::new(not_left), Box::new(not_right.clone()));
                let eu = AstNode::EU(Box::new(not_right.clone()), Box::new(conj));
                let eg = AstNode::EG(Box::new(not_right));
                AstNode::Not(Box::new(AstNode::Or(Box::new(eu), Box::new(eg))))
            }
            AstNode::EU(lhs, rhs) => {
                AstNode::EU(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            AstNode::Id(_) | AstNode::True | AstNode::False => {
                self // 基础元素，直接返回
            }
        }
    }

    fn optimize(self) -> AstNode {
        match self {
            // 1. Double negation elimination
            AstNode::Not(inner) => {
                match *inner {
                    AstNode::Not(inner2) => inner2.optimize(), 
                    other => AstNode::Not(Box::new(other.optimize())),
                }
            }

            // 2. Simplify And
            AstNode::And(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                match (&left, &right) {
                    (AstNode::True, _) => right,
                    (_, AstNode::True) => left,
                    (AstNode::False, _) => AstNode::False,
                    (_, AstNode::False) => AstNode::False,
                    _ => AstNode::And(Box::new(left), Box::new(right)),
                }
            }

            // 3. Simplify Or
            AstNode::Or(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                match (&left, &right) {
                    (AstNode::True, _) => AstNode::True,
                    (_, AstNode::True) => AstNode::True,
                    (AstNode::False, _) => right,
                    (_, AstNode::False) => left,
                    _ => AstNode::Or(Box::new(left), Box::new(right)),
                }
            }

            AstNode::Implies(left, right) => {
                AstNode::Implies(Box::new(left.optimize()), Box::new(right.optimize()))
            }

            AstNode::EG(inner) => AstNode::EG(Box::new(inner.optimize())),
            AstNode::EX(inner) => AstNode::EX(Box::new(inner.optimize())),
            AstNode::EU(left, right) => {
                AstNode::EU(Box::new(left.optimize()), Box::new(right.optimize()))
            }

            AstNode::Id(_) | AstNode::True | AstNode::False => self,
            AstNode::AF(_) | AstNode::AG(_) | AstNode::AX(_) | AstNode::AU(_, _) | AstNode::EF(_) => {
                panic!("Normalization failed, {:?} is not deleted.", self);
            }
        }
    }
}
