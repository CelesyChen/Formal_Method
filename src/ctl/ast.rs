use core::panic;

#[derive(Debug, Clone)]
pub enum CtlAst {
    And(Box<CtlAst>, Box<CtlAst>),
    Or(Box<CtlAst>, Box<CtlAst>),
    Implies(Box<CtlAst>, Box<CtlAst>),
    Not(Box<CtlAst>),
    AG(Box<CtlAst>),
    EG(Box<CtlAst>),
    AX(Box<CtlAst>),
    EX(Box<CtlAst>),
    AF(Box<CtlAst>),
    EF(Box<CtlAst>),
    AU(Box<CtlAst>, Box<CtlAst>),
    EU(Box<CtlAst>, Box<CtlAst>),
    Expr(String, Atom),
    True,
    False,
}

#[derive(Debug, Clone)]
pub enum Atom {
    Id(String),
    Num(u32)
}

impl CtlAst {
    pub fn norm_and_opt(self) -> CtlAst {
        self.normalize().optimize()
    }

    fn normalize(self) -> CtlAst {
        match self {
            CtlAst::And(lhs, rhs) => {
                CtlAst::And(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            CtlAst::Or(lhs, rhs) => {
                CtlAst::Or(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            CtlAst::Implies(lhs, rhs) => {
                CtlAst::Implies(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            CtlAst::Not(inner) => {
                CtlAst::Not(Box::new(inner.normalize()))
            }
            CtlAst::AG(inner) => {
                // AG x = not EF not x = not E [ T U not x]
                CtlAst::Not(
                    Box::new(CtlAst::EU(
                        Box::new(CtlAst::True), 
                        Box::new(CtlAst::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            CtlAst::EG(inner) => {
                CtlAst::EG(Box::new(inner.normalize()))
            }
            CtlAst::AX(inner) => {
                // AX x = not EX not x
                CtlAst::Not(
                    Box::new(CtlAst::EX(
                        Box::new(CtlAst::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            CtlAst::EX(inner) => {
                CtlAst::EX(Box::new(inner.normalize()))
            }
            CtlAst::AF(inner) => {
                // AF x = not EG not x
                CtlAst::Not(
                    Box::new(CtlAst::EG(
                        Box::new(CtlAst::Not(
                            Box::new(inner.normalize())
                        ))
                    ))
                )
            }
            CtlAst::EF(inner) => {
                // EF x = E[T U x]
                CtlAst::EU(Box::new(CtlAst::True), 
                    Box::new(inner.normalize())
                )
            }
            CtlAst::AU(left, right) => {
                // A[φ U ψ]  -->  ¬(E[¬ψ U (¬φ ∧ ¬ψ)] ∨ EG¬ψ)
                let not_right = CtlAst::Not(Box::new(right.normalize()));
                let not_left = CtlAst::Not(Box::new(left.normalize()));
                let conj = CtlAst::And(Box::new(not_left), Box::new(not_right.clone()));
                let eu = CtlAst::EU(Box::new(not_right.clone()), Box::new(conj));
                let eg = CtlAst::EG(Box::new(not_right));
                CtlAst::Not(Box::new(CtlAst::Or(Box::new(eu), Box::new(eg))))
            }
            CtlAst::EU(lhs, rhs) => {
                CtlAst::EU(Box::new(lhs.normalize()), Box::new(rhs.normalize()))
            }
            CtlAst::Expr(..) | CtlAst::True | CtlAst::False => {
                self // 基础元素，直接返回
            }
        }
    }

    fn optimize(self) -> CtlAst {
        match self {
            // 1. Double negation elimination
            CtlAst::Not(inner) => {
                match *inner {
                    CtlAst::Not(inner2) => inner2.optimize(), 
                    other => CtlAst::Not(Box::new(other.optimize())),
                }
            }

            // 2. Simplify And
            CtlAst::And(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                match (&left, &right) {
                    (CtlAst::True, _) => right,
                    (_, CtlAst::True) => left,
                    (CtlAst::False, _) => CtlAst::False,
                    (_, CtlAst::False) => CtlAst::False,
                    _ => CtlAst::And(Box::new(left), Box::new(right)),
                }
            }

            // 3. Simplify Or
            CtlAst::Or(left, right) => {
                let left = left.optimize();
                let right = right.optimize();
                match (&left, &right) {
                    (CtlAst::True, _) => CtlAst::True,
                    (_, CtlAst::True) => CtlAst::True,
                    (CtlAst::False, _) => right,
                    (_, CtlAst::False) => left,
                    _ => CtlAst::Or(Box::new(left), Box::new(right)),
                }
            }

            CtlAst::Implies(left, right) => {
                CtlAst::Implies(Box::new(left.optimize()), Box::new(right.optimize()))
            }

            CtlAst::EG(inner) => CtlAst::EG(Box::new(inner.optimize())),
            CtlAst::EX(inner) => CtlAst::EX(Box::new(inner.optimize())),
            CtlAst::EU(left, right) => {
                CtlAst::EU(Box::new(left.optimize()), Box::new(right.optimize()))
            }

            CtlAst::Expr(..) | CtlAst::True | CtlAst::False => self,
            CtlAst::AF(_) | CtlAst::AG(_) | CtlAst::AX(_) | CtlAst::AU(_, _) | CtlAst::EF(_) => {
                panic!("Normalization failed, {:?} is not deleted.", self);
            }
        }
    }
}
