use crate::{Prop, StepIndex};
use std::fmt;

impl fmt::Display for StepIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Inference rules from from page 27 in "Logic in Computer Science" by Huth and Ryan
#[derive(Debug, Clone, PartialEq, Eq)]
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
            Rule::AndI(phi, psi) => write!(f, "∧I {phi} {psi}"),
            Rule::AndELhs(phi) => write!(f, "∧E_lhs {phi}"),
            Rule::AndERhs(phi) => write!(f, "∧E_rhs {phi}"),
            Rule::OrILhs(phi, psi) => write!(f, "∨I_lhs {phi} {psi}"),
            Rule::OrIRhs(phi, psi) => write!(f, "∨I_rhs {phi} {psi}"),
            Rule::OrE {
                or_prop,
                lhs_box,
                rhs_box,
            } => write!(f, "∨E {or_prop} {lhs_box} {rhs_box}"),
            Rule::NegI(phi) => write!(f, "¬I {phi}"),
            Rule::NegE { prop, neg_prop } => write!(f, "¬E {prop} {neg_prop}"),
            Rule::ImplyI(phi) => write!(f, "→I {phi}"),
            Rule::ImplyE {
                implication,
                lhs_proof,
            } => write!(f, "→E {implication} {lhs_proof}"),
            Rule::BottomE(phi, psi) => write!(f, "⊥E {phi} {psi}"),
            Rule::DoubleNegE(phi) => write!(f, "¬¬E {phi}"),
            Rule::ModusTollens {
                implication,
                negated_rhs,
            } => write!(f, "MT {implication} {negated_rhs}"),
            Rule::DoubleNegI(phi) => write!(f, "¬¬I {phi}"),
            Rule::ProofByContradiction(phi) => write!(f, "PBC {phi}"),
            Rule::LawOfExcludedMiddle(phi) => write!(f, "LEM {phi}"),
        }
    }
}
