use std::{collections::HashMap, ops::RangeFrom};

use crate::{Error, Prop, PropVariant, Rule, Step, StepIndex, StepType, SubProof};

#[derive(Debug)]
pub(crate) struct Scope {
    pub(crate) steps: HashMap<StepIndex, Step>,
}

impl Scope {
    fn new() -> Self {
        Self {
            steps: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Proof {
    pub(crate) context: Vec<Scope>,
    pub(crate) index_counter: RangeFrom<usize>,
}

impl Default for Proof {
    fn default() -> Self {
        Self::new()
    }
}

impl Proof {
    pub fn new() -> Self {
        Self {
            context: vec![Scope::new()],
            index_counter: (1usize..),
        }
    }

    pub fn copy(&mut self, index: StepIndex) -> Result<StepIndex, Error> {
        let prop = self.get_prop(index)?;
        Ok(self.add_step(Step::new(prop.clone(), StepType::Copy(index))))
    }

    pub fn add_premise(&mut self, premise: Prop) -> StepIndex {
        // TODO: ensure that we only can add premises in the beginning
        let index = self.next_index();
        self.context
            .last_mut()
            .unwrap()
            .steps
            .insert(index, Step::new(premise.clone(), StepType::Premise));

        index
    }

    /// Introduce a new scope and add an assumption to it
    pub fn add_assumption(&mut self, assumption: Prop) -> StepIndex {
        self.context.push(Scope::new());
        self.add_step(Step::new(assumption, StepType::Assumption))
    }

    // TODO: Maybe a more suitable name would be "dispatch_assumption"
    // TODO: Proper error handling
    /// Close the current scope and inserts a "proof box" (with the assumption and the derived proposition)
    /// in the upper scope with the same index as the starting index of the closed scope
    pub fn close_scope(&mut self) -> Result<(), Error> {
        if self.context.len() == 1 {
            return Err(Error::CannotCloseGlobalScope);
        }

        let mut scope = self.context.pop().unwrap();
        let subproof = SubProof::new(scope.steps.drain().collect());
        let starting_index = subproof.starting_index();
        let proof_box = Prop::ProofBox(subproof);

        self.context
            .last_mut()
            .unwrap()
            .steps
            .insert(starting_index, Step::new(proof_box, StepType::Assumption));

        Ok(())
    }

    fn next_index(&mut self) -> StepIndex {
        StepIndex(self.index_counter.next().unwrap())
    }

    fn add_step(&mut self, step: Step) -> StepIndex {
        let index = self.next_index();
        let scope = self.context.last_mut().unwrap();
        scope.steps.insert(index, step);
        index
    }

    pub fn get_prop(&self, index: StepIndex) -> Result<&Prop, Error> {
        self.get_step(index).map(|step| step.prop())
    }

    pub fn get_step(&self, index: StepIndex) -> Result<&Step, Error> {
        self.get_step_helper(self.context.len() - 1, index)
    }

    fn get_step_helper(&self, scope_level: usize, index: StepIndex) -> Result<&Step, Error> {
        let Some(scope) = self.context.get(scope_level) else {
            return Err(Error::InvalidStepIndex { index });
        };

        if let Some(step) = scope.steps.get(&index) {
            Ok(step)
        } else if scope_level > 0 {
            self.get_step_helper(scope_level - 1, index)
        } else {
            return Err(Error::InvalidStepIndex { index });
        }
    }

    /// Apply an inference rule using the current context and scope
    pub fn apply_rule(&mut self, rule: &Rule) -> Result<StepIndex, Error> {
        match rule {
            Rule::AndI(lhs_index, rhs_index) => {
                let lhs = self.get_prop(*lhs_index)?;
                let rhs = self.get_prop(*rhs_index)?;
                let prop = Prop::And(Box::new(lhs.clone()), Box::new(rhs.clone()));
                Ok(self.add_step(Step::new(prop, StepType::Rule(rule.clone()))))
            }
            Rule::AndELhs(prop_index) => {
                let prop = self.get_prop(*prop_index)?;
                if let Prop::And(lhs, _) = prop {
                    let prop = *lhs.clone();
                    Ok(self.add_step(Step::new(prop, StepType::Rule(rule.clone()))))
                } else {
                    Err(Error::ExpectedPropVariant {
                        expected: PropVariant::And,
                        got: prop.clone(),
                    })
                }
            }
            Rule::AndERhs(prop_index) => {
                let prop = self.get_prop(*prop_index)?;
                if let Prop::And(_, rhs) = prop {
                    let prop = *rhs.clone();
                    Ok(self.add_step(Step::new(prop, StepType::Rule(rule.clone()))))
                } else {
                    Err(Error::ExpectedPropVariant {
                        expected: PropVariant::And,
                        got: prop.clone(),
                    })
                }
            }
            Rule::ImplyI(proof_box) => {
                let proof_box_prop = self.get_prop(*proof_box)?;
                let Prop::ProofBox(subproof) = proof_box_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box_prop.clone(),
                    });
                };

                let prop = Prop::Imply(
                    Box::new(subproof.assumption().clone()),
                    Box::new(subproof.derived_prop().clone()),
                );
                Ok(self.add_step(Step::new(prop, StepType::Rule(rule.clone()))))
            }
            Rule::OrILhs(index, other) => {
                let prop = self.get_prop(*index)?;
                let new = Prop::Or(Box::new(prop.clone()), Box::new(other.clone()));
                Ok(self.add_step(Step::new(new, StepType::Rule(rule.clone()))))
            }
            Rule::OrIRhs(other, index) => {
                let prop = self.get_prop(*index)?;
                let new = Prop::Or(Box::new(other.clone()), Box::new(prop.clone()));
                Ok(self.add_step(Step::new(new, StepType::Rule(rule.clone()))))
            }
            Rule::OrE {
                or_prop,
                lhs_box,
                rhs_box,
            } => {
                let or_prop = self.get_prop(*or_prop)?;
                let lhs_box = self.get_prop(*lhs_box)?;
                let rhs_box = self.get_prop(*rhs_box)?;

                let Prop::Or(or_lhs, or_rhs) = or_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: or_prop.clone(),
                    });
                };

                let Prop::ProofBox(lhs_subproof) = lhs_box else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: lhs_box.clone(),
                    });
                };

                let Prop::ProofBox(rhs_subproof) = lhs_box else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: rhs_box.clone(),
                    });
                };

                check_eq(or_lhs, lhs_subproof.assumption())?;
                check_eq(or_rhs, rhs_subproof.assumption())?;
                check_eq(lhs_subproof.derived_prop(), rhs_subproof.derived_prop())?;

                Ok(self.add_step(Step::new(
                    lhs_subproof.derived_prop().clone(),
                    StepType::Rule(rule.clone()),
                )))
            }
            Rule::NegI(proof_box) => {
                let proof_box = self.get_prop(*proof_box)?;

                let Prop::ProofBox(subproof) = proof_box else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box.clone(),
                    });
                };

                check_eq(subproof.derived_prop(), &Prop::Bottom)?;

                let negated_prop = Prop::Imply(
                    Box::new(subproof.assumption().clone()),
                    Box::new(Prop::Bottom),
                );
                Ok(self.add_step(Step::new(negated_prop, StepType::Rule(rule.clone()))))
            }
            Rule::NegE { prop, neg_prop } => {
                let prop = self.get_prop(*prop)?;
                let neg_prop = self.get_prop(*neg_prop)?;

                // neg prop := prop -> bottom
                let Prop::Imply(lhs, rhs) = neg_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: prop.clone(),
                    });
                };
                check_eq(lhs, prop)?;
                check_eq(rhs, &Prop::Bottom)?;

                Ok(self.add_step(Step::new(Prop::Bottom, StepType::Rule(rule.clone()))))
            }
            Rule::BottomE(bottom_prop, prop) => {
                let bottom_prop = self.get_prop(*bottom_prop)?;
                check_eq(bottom_prop, &Prop::Bottom)?;
                Ok(self.add_step(Step::new(prop.clone(), StepType::Rule(rule.clone()))))
            }
            Rule::DoubleNegE(double_negated_prop) => {
                let double_negated_prop = self.get_prop(*double_negated_prop)?;

                // neg (neg phi) := (neg phi) -> bottom = (phi -> bottom) -> bottom
                let Prop::Imply(ref prop_bottom, ref bottom) = double_negated_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: double_negated_prop.clone(),
                    });
                };

                check_eq(bottom, &Prop::Bottom)?;

                let Prop::Imply(ref prop, ref bottom2) = **prop_bottom else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: *prop_bottom.clone(),
                    });
                };

                check_eq(bottom2, &Prop::Bottom)?;

                Ok(self.add_step(Step::new(*prop.clone(), StepType::Rule(rule.clone()))))
            }
            Rule::ImplyE {
                implication,
                lhs_proof,
            } => {
                let implication = self.get_prop(*implication)?;
                let lhs_proof = self.get_prop(*lhs_proof)?;

                let Prop::Imply(lhs, rhs) = implication else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: implication.clone(),
                    });
                };

                check_eq(lhs, lhs_proof)?;

                Ok(self.add_step(Step::new(*rhs.clone(), StepType::Rule(rule.clone()))))
            }
            Rule::ModusTollens {
                implication,
                negated_rhs,
            } => {
                let implication = self.get_prop(*implication)?;
                let negated_rhs = self.get_prop(*negated_rhs)?;

                let Prop::Imply(lhs, rhs) = implication else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: implication.clone(),
                    });
                };

                check_eq(negated_rhs, &Prop::negated(*rhs.clone()))?;

                let neg_lhs = Prop::negated(*lhs.clone());
                Ok(self.add_step(Step::new(neg_lhs, StepType::Rule(rule.clone()))))
            }
            Rule::DoubleNegI(prop) => {
                let prop = self.get_prop(*prop)?;
                let neg_neg_prop = Prop::negated(Prop::negated(prop.clone()));
                Ok(self.add_step(Step::new(neg_neg_prop, StepType::Rule(rule.clone()))))
            }
            Rule::ProofByContradiction(proof_box) => {
                let proof_box = self.get_prop(*proof_box)?;

                let Prop::ProofBox(subproof) = proof_box else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box.clone(),
                    });
                };

                // check if the assumption is negated (i.e. has the form phi -> bottom)
                let Prop::Imply(ref lhs, ref rhs) = subproof.assumption() else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: subproof.assumption().clone(),
                    });
                };
                check_eq(rhs, &Prop::Bottom)?;

                // also check that the proof box ends with bottom
                check_eq(subproof.derived_prop(), &Prop::Bottom)?;

                Ok(self.add_step(Step::new(*lhs.clone(), StepType::Rule(rule.clone()))))
            }
            Rule::LawOfExcludedMiddle(prop) => {
                let neg_prop = Prop::negated(prop.clone());
                let or_prop = Prop::Or(Box::new(prop.clone()), Box::new(neg_prop));
                Ok(self.add_step(Step::new(or_prop, StepType::Rule(rule.clone()))))
            }
        }
    }

    pub fn undo(&mut self) {
        let next_index = self.index_counter.next().unwrap();

        // If we don't have any steps, we can't undo anything, except restart at 1
        if next_index == 1 {
            self.index_counter = 1..;
            return;
        }

        let latest_index = next_index - 1;
        // Revert the index counter
        self.index_counter = latest_index..;

        let latest_step_type = self
            .get_step(StepIndex(latest_index))
            .unwrap()
            .step_type()
            .clone();

        match latest_step_type {
            StepType::Rule(_) | StepType::Copy(_) | StepType::Premise => {
                let scope = self.context.last_mut().unwrap();
                scope.steps.remove(&StepIndex(latest_index));
            }
            StepType::Assumption => {
                // remove the entire scope
                self.context.pop();
            }
        }
    }
}

fn check_eq(p: &Prop, q: &Prop) -> Result<(), Error> {
    if p != q {
        return Err(Error::PropMismatch {
            expected: p.clone(),
            got: q.clone(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjunction_introduction() {
        /*
        p   premise
        q   premise
        p^q conj_i 1,2
        */
        let mut ctx = Proof::new();
        let p = ctx.add_premise(Prop::Symbol("p".into()));
        let q = ctx.add_premise(Prop::Symbol("q".into()));
        let p_and_q_prop = ctx.apply_rule(&Rule::AndI(p, q)).unwrap();

        assert_eq!(
            ctx.get_prop(p_and_q_prop).unwrap(),
            &Prop::And(
                Box::new(Prop::Symbol("p".into())),
                Box::new(Prop::Symbol("q".into()))
            )
        )
    }

    #[test]
    fn scope_test() {
        /*
        1. q   premise
        ----------------
        2. p   assume
        3. q   copy
        ----------------
        4. p -> q ->e 1, 2
        */

        let mut ctx = Proof::new();
        let q = ctx.add_premise(Prop::Symbol("q".into()));
        let p = ctx.add_assumption(Prop::Symbol("p".into()));
        let _ = ctx.copy(q).unwrap();
        ctx.close_scope().unwrap();
        let p_implies_q_prop = ctx.apply_rule(&Rule::ImplyI(p)).unwrap();

        assert_eq!(
            ctx.get_prop(p_implies_q_prop).unwrap(),
            &Prop::Imply(
                Box::new(Prop::Symbol("p".into())),
                Box::new(Prop::Symbol("q".into())),
            )
        );
    }
}
