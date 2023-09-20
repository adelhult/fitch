use crate::Rule;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prop {
    Bottom,
    Symbol(String),
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Imply(Box<Prop>, Box<Prop>),
    ProofBox(SubProof),
}

// TODO: Would be really nice if a subproof could uphold the invariant that it must start with an assumption step
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubProof(pub(crate) Vec<(StepIndex, Step)>);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct StepIndex(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step(Prop, StepType);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepType {
    Rule(Rule),
    Copy(StepIndex),
    Premise,
    Assumption,
}

#[derive(Clone, Debug)]
pub enum PropVariant {
    Bottom,
    Symbol,
    And,
    Or,
    Imply,
    ProofBox,
}

impl Prop {
    pub fn negated(prop: Self) -> Self {
        Prop::Imply(Box::new(prop), Box::new(Prop::Bottom))
    }
}

impl SubProof {
    pub fn assumption(&self) -> &Prop {
        &self.0[0].1.prop() // TODO: handle empty subproofs
    }

    pub fn derived_prop(&self) -> &Prop {
        &self.0.last().unwrap().1.prop() // TODO: handle empty subproofs
    }

    pub fn starting_index(&self) -> StepIndex {
        self.0[0].0 // TODO: handle empty subproofs
    }
}

impl Step {
    pub fn new(prop: Prop, step_type: StepType) -> Self {
        Self(prop, step_type)
    }

    pub fn prop_owned(self) -> Prop {
        self.0
    }

    pub fn prop(&self) -> &Prop {
        &self.0
    }

    pub fn step_type_owned(self) -> StepType {
        self.1
    }

    pub fn step_type(&self) -> &StepType {
        &self.1
    }
}

impl From<&Prop> for PropVariant {
    fn from(prop: &Prop) -> Self {
        match prop {
            Prop::Bottom => PropVariant::Bottom,
            Prop::Symbol(..) => PropVariant::Symbol,
            Prop::And(..) => PropVariant::And,
            Prop::Or(..) => PropVariant::Or,
            Prop::Imply(..) => PropVariant::Imply,
            Prop::ProofBox { .. } => PropVariant::ProofBox,
        }
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Prop::*;
        match self {
            Bottom => write!(f, "⊥"),
            Symbol(s) => write!(f, "{s}"),
            And(lhs, rhs) => {
                match &**lhs {
                    lhs @ Imply(..) | lhs @ And(..) | lhs @ Or(..) => write!(f, "({lhs})")?,
                    _ => write!(f, "{lhs}")?,
                }

                write!(f, " ∧ ")?;

                match &**rhs {
                    rhs @ Imply(..) | rhs @ And(..) | rhs @ Or(..) => write!(f, "({rhs})")?,
                    _ => write!(f, "{rhs}")?,
                }

                Ok(())
            }

            Or(lhs, rhs) => {
                match &**lhs {
                    lhs @ Imply(..) | lhs @ And(..) | lhs @ Or(..) => write!(f, "({lhs})")?,
                    _ => write!(f, "{lhs}")?,
                }

                write!(f, " ∨ ")?;

                match &**rhs {
                    rhs @ Imply(..) | rhs @ And(..) | rhs @ Or(..) => write!(f, "({rhs})")?,
                    _ => write!(f, "{rhs}")?,
                }

                Ok(())
            }

            Imply(lhs, rhs) => match &**lhs {
                lhs @ Imply(..) => write!(f, "({lhs}) → {rhs}"),
                _ => write!(f, "{lhs} → {rhs}"),
            },

            ProofBox(subproof) => write!(
                f,
                "[{assumption}... {derived_prop}]",
                assumption = subproof.assumption(),
                derived_prop = subproof.derived_prop()
            ),
        }
    }
}
