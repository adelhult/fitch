use crate::Prop;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct StepIndex(pub usize);

/// Inference rules from from page 27 in "Logic in Computer Science" by Huth and Ryan
#[derive(Debug)]
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
