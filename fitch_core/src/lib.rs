mod error;
mod latex;
mod pretty_print_ascii;
mod proof;
mod prop;
mod rules;

pub use error::Error;
pub use latex::latex;
pub use pretty_print_ascii::print_proof;
pub use proof::Proof;
pub use prop::{Prop, PropVariant, Step, StepIndex, StepType, SubProof};
pub use rules::{Rule, RuleName};
