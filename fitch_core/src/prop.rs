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
