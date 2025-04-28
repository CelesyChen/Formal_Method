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
    pub fn normalize(self) -> AstNode {
        match self {
            AstNode::AF(inner) => {
                // AF φ  -->  ¬ EG ¬ φ
                AstNode::Not(Box::new(
                    AstNode::EG(Box::new(
                        AstNode::Not(Box::new(inner.normalize()))
                    ))
                ))
            }
            AstNode::EF(inner) => {
                // EF φ  -->  ¬ AG ¬ φ
                AstNode::EU(Box::new(AstNode::True), 
                    Box::new(inner.normalize()))
            }
            AstNode::AX(inner) => {
                // AX φ  -->  ¬ EX ¬ φ
                AstNode::Not(Box::new(
                    AstNode::EX(Box::new(
                        AstNode::Not(Box::new(inner.normalize()))
                    ))
                ))
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
                AstNode::AG(Box::new(inner.normalize()))
            }
            AstNode::EG(inner) => {
                AstNode::EG(Box::new(inner.normalize()))
            }
            AstNode::EX(inner) => {
                AstNode::EX(Box::new(inner.normalize()))
            }
            AstNode::EU(lhs, rhs) => {
                AstNode::EU(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            AstNode::Id(_) | AstNode::True | AstNode::False => {
                self // 基础元素，直接返回
            }
        }
    }
}
