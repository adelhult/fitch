mod error;
mod proof;
mod prop;
mod rules;

pub use error::Error;
pub use proof::Proof;
pub use prop::{Prop, PropVariant, Step, StepIndex, StepType, SubProof};
pub use rules::Rule;
