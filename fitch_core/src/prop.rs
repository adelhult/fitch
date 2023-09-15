use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Prop {
    Bottom,
    Symbol(String),
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Imply(Box<Prop>, Box<Prop>),
    ProofBox {
        assumption: Box<Prop>,
        derived_prop: Box<Prop>,
    },
}

impl Prop {
    pub fn negated(prop: Self) -> Self {
        Prop::Imply(Box::new(prop), Box::new(Prop::Bottom))
    }
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

            ProofBox {
                assumption,
                derived_prop,
            } => write!(f, "[{assumption}... {derived_prop}]"),
        }
    }
}
