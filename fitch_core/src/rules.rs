use crate::{Prop, StepIndex};
use std::fmt;

impl fmt::Display for StepIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Inference rules from from page 27 in "Logic in Computer Science" by Huth and Ryan
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rule {
    /// ```notrust
    /// phi     psi
    /// -----------
    /// phi  ^  psi
    /// ```
    AndI(StepIndex, StepIndex),

    /// ```notrust
    /// phi  ^  psi
    /// -----------
    ///     phi
    /// ```
    AndELhs(StepIndex),

    /// ```notrust
    /// phi  ^  psi
    /// -----------
    ///     psi
    /// ```
    AndERhs(StepIndex),

    /// ```notrust
    ///     phi
    /// ------------
    ///  phi \/ psi
    /// ```
    OrILhs(StepIndex, Prop),

    /// ```notrust
    ///     psi
    /// ------------
    ///  phi \/ psi
    /// ```
    OrIRhs(Prop, StepIndex),

    /// ```notrust
    /// phi \/ psi   [phi]...chi   [psi]...chi
    /// -------------------------------------
    ///                chi
    /// ```
    OrE {
        or_prop: StepIndex,
        lhs_box: StepIndex,
        rhs_box: StepIndex,
    },

    /// ```notrust
    ///  [phi]...bottom
    /// ----------------
    ///     neg phi
    /// ```
    /// (Note: neg phi = phi -> bottom)
    NegI(StepIndex),

    /// ```notrust
    ///  phi    neg phi
    /// ----------------
    ///      bottom
    /// ```
    NegE {
        prop: StepIndex,
        neg_prop: StepIndex,
    },

    /// ```notrust
    ///  [phi]...psi
    /// -------------
    ///      psi
    /// ```
    ImplyI(StepIndex),

    /// Modus ponens
    /// ```notrust
    ///  phi      phi -> psi
    /// ---------------------
    ///          psi
    /// ```
    ImplyE {
        implication: StepIndex,
        lhs_proof: StepIndex,
    },

    /// ```notrust
    ///    bottom
    /// ------------
    ///     phi
    /// ```
    BottomE(StepIndex, Prop),

    /// ```notrust
    ///  neg (neg phi)
    /// ---------------
    ///      phi
    /// ```
    DoubleNegE(StepIndex),

    /// ```notrust
    ///  phi -> psi    neg psi
    /// ------------------------
    ///          neg phi
    /// ```
    ModusTollens {
        implication: StepIndex,
        negated_rhs: StepIndex,
    },

    /// ```notrust
    ///       phi
    /// ---------------
    ///  neg (neg phi)
    /// ```
    DoubleNegI(StepIndex),

    /// ```notrust
    ///  [not phi]...bottom
    /// --------------------
    ///         phi
    /// ```
    ProofByContradiction(StepIndex),

    /// ```notrust
    /// ----------------
    ///  phi \/ neg phi
    /// ```
    LawOfExcludedMiddle(Prop),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::AndI(phi, psi) => {
                write!(f, "{name} {phi} {psi}", name = Into::<RuleName>::into(self))
            }
            Rule::AndELhs(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::AndERhs(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::OrILhs(phi, psi) => {
                write!(f, "{name} {phi} {psi}", name = Into::<RuleName>::into(self))
            }
            Rule::OrIRhs(phi, psi) => {
                write!(f, "{name} {phi} {psi}", name = Into::<RuleName>::into(self))
            }
            Rule::OrE {
                or_prop,
                lhs_box,
                rhs_box,
            } => write!(
                f,
                "{name} {or_prop} {lhs_box} {rhs_box}",
                name = Into::<RuleName>::into(self)
            ),
            Rule::NegI(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::NegE { prop, neg_prop } => write!(
                f,
                "{name} {prop} {neg_prop}",
                name = Into::<RuleName>::into(self)
            ),
            Rule::ImplyI(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::ImplyE {
                implication,
                lhs_proof,
            } => write!(
                f,
                "{name} {implication} {lhs_proof}",
                name = Into::<RuleName>::into(self)
            ),
            Rule::BottomE(phi, psi) => {
                write!(f, "{name} {phi} {psi}", name = Into::<RuleName>::into(self))
            }
            Rule::DoubleNegE(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::ModusTollens {
                implication,
                negated_rhs,
            } => write!(
                f,
                "{name} {implication} {negated_rhs}",
                name = Into::<RuleName>::into(self)
            ),
            Rule::DoubleNegI(phi) => write!(f, "{name} {phi}", name = Into::<RuleName>::into(self)),
            Rule::ProofByContradiction(phi) => {
                write!(f, "{name} {phi}", name = Into::<RuleName>::into(self))
            }
            Rule::LawOfExcludedMiddle(phi) => {
                write!(f, "{name} {phi}", name = Into::<RuleName>::into(self))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleName {
    AndI,
    AndELhs,
    AndERhs,
    OrILhs,
    OrIRhs,
    OrE,
    NegI,
    NegE,
    ImplyI,
    ImplyE,
    BottomE,
    DoubleNegE,
    ModusTollens,
    DoubleNegI,
    ProofByContradiction,
    LawOfExcludedMiddle,
}

impl From<&Rule> for RuleName {
    fn from(rule: &Rule) -> Self {
        match rule {
            Rule::AndI(..) => RuleName::AndI,
            Rule::AndELhs(..) => RuleName::AndELhs,
            Rule::AndERhs(..) => RuleName::AndERhs,
            Rule::OrILhs(..) => RuleName::OrILhs,
            Rule::OrIRhs(..) => RuleName::OrIRhs,
            Rule::OrE { .. } => RuleName::OrE,
            Rule::NegI(..) => RuleName::NegI,
            Rule::NegE { .. } => RuleName::NegE,
            Rule::ImplyI(..) => RuleName::ImplyI,
            Rule::ImplyE { .. } => RuleName::ImplyE,
            Rule::BottomE(..) => RuleName::BottomE,
            Rule::DoubleNegE(..) => RuleName::DoubleNegE,
            Rule::ModusTollens { .. } => RuleName::ModusTollens,
            Rule::DoubleNegI(..) => RuleName::DoubleNegI,
            Rule::ProofByContradiction(..) => RuleName::ProofByContradiction,
            Rule::LawOfExcludedMiddle(..) => RuleName::LawOfExcludedMiddle,
        }
    }
}

impl fmt::Display for RuleName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RuleName::*;
        match self {
            AndI => write!(f, "∧I"),
            AndELhs => write!(f, "∧E_lhs"),
            AndERhs => write!(f, "∧E_rhs"),
            OrILhs => write!(f, "∨I_lhs"),
            OrIRhs => write!(f, "∨I_rhs"),
            OrE => write!(f, "∨E"),
            NegI => write!(f, "¬I"),
            NegE => write!(f, "¬E"),
            ImplyI => write!(f, "→I"),
            ImplyE => write!(f, "→E"),
            BottomE => write!(f, "⊥E"),
            DoubleNegE => write!(f, "¬¬E"),
            ModusTollens => write!(f, "MT"),
            DoubleNegI => write!(f, "¬¬I"),
            ProofByContradiction => write!(f, "PBC"),
            LawOfExcludedMiddle => write!(f, "LEM"),
        }
    }
}
