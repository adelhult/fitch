use crate::{Prop, PropVariant, StepIndex};

// TODO: Replace Prop and PropVariant Debug with Display
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I expected '{expected:?}' but you gave me '{got:?}'")]
    PropMismatch { expected: Prop, got: Prop },
    #[error("You used an invalid index '{0}'", index.0)]
    InvalidStepIndex { index: StepIndex },
    #[error("I expected a '{expected:?}' but got the expression '{got:?}'.")]
    ExpectedPropVariant { expected: PropVariant, got: Prop },
}
