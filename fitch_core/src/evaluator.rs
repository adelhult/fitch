use std::{collections::HashMap, ops::RangeFrom};

use crate::{Error, Prop, PropVariant, Rule, StepIndex};

#[derive(Debug)]
struct Scope {
    props: HashMap<StepIndex, Prop>,
}

impl Scope {
    fn new() -> Self {
        Self {
            props: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    scopes: Vec<Scope>,
    index_counter: RangeFrom<usize>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::new()],
            index_counter: (0usize..),
        }
    }

    pub fn copy(&mut self, index: StepIndex) -> Result<StepIndex, Error> {
        let prop = self.get_prop(index)?;
        Ok(self.add_step(prop.clone()))
    }

    pub fn add_premise(&mut self, premise: Prop) -> StepIndex {
        // TODO: ensure that we only can add premises in the beginning
        let index = self.next_index();
        self.scopes.last_mut().unwrap().props.insert(index, premise);

        index
    }

    /// Introduce a new scope and add an assumption to it
    pub fn add_assumption(&mut self, assumption: Prop) -> StepIndex {
        self.scopes.push(Scope::new());
        self.add_step(assumption)
    }

    // TODO: Maybe a more suitable name would be "dispatch_assumption"
    // TODO: Proper error handling
    /// Close the current scope and inserts a "proof box" (with the assumption and the derived proposition)
    /// in the upper scope with the same index as the starting index of the closed scope
    pub fn close_scope(&mut self) -> Result<(), ()> {
        let mut scope = self.scopes.pop().ok_or(())?;
        let mut props = scope.props.drain();
        let (starting_index, assumption) = props.nth(0).unwrap();
        let (_, derived_prop) = props.last().unwrap();

        let proof_box = Prop::ProofBox {
            assumption: Box::new(assumption),
            derived_prop: Box::new(derived_prop),
        };

        self.scopes
            .last_mut()
            .ok_or(())?
            .props
            .insert(starting_index, proof_box);

        Ok(())
    }

    fn next_index(&mut self) -> StepIndex {
        StepIndex(self.index_counter.next().unwrap())
    }

    fn add_step(&mut self, prop: Prop) -> StepIndex {
        let index = self.next_index();
        let scope = self.scopes.last_mut().unwrap();
        scope.props.insert(index, prop);
        index
    }

    pub fn get_prop(&self, index: StepIndex) -> Result<&Prop, Error> {
        self.get_step_helper(self.scopes.len() - 1, index)
    }

    fn get_step_helper(&self, scope_level: usize, index: StepIndex) -> Result<&Prop, Error> {
        let Some(scope) = self.scopes.get(scope_level) else {
            return Err(Error::InvalidStepIndex { index });
        };

        if let Some(prop) = scope.props.get(&index) {
            Ok(prop)
        } else if scope_level > 0 {
            self.get_step_helper(scope_level - 1, index)
        } else {
            return Err(Error::InvalidStepIndex { index });
        }
    }

    /// Apply an inference rule using the current context and scope
    pub fn apply_rule(&mut self, rule: Rule) -> Result<StepIndex, Error> {
        match rule {
            Rule::AndI(lhs_index, rhs_index) => {
                let lhs = self.get_prop(lhs_index)?;
                let rhs = self.get_prop(rhs_index)?;
                let prop = Prop::And(Box::new(lhs.clone()), Box::new(rhs.clone()));
                Ok(self.add_step(prop))
            }
            Rule::AndELhs(prop_index) => {
                let prop = self.get_prop(prop_index)?;
                if let Prop::And(lhs, _) = prop {
                    let prop = *lhs.clone();
                    Ok(self.add_step(prop))
                } else {
                    Err(Error::ExpectedPropVariant {
                        expected: PropVariant::And,
                        got: prop.clone(),
                    })
                }
            }
            Rule::AndERhs(prop_index) => {
                let prop = self.get_prop(prop_index)?;
                if let Prop::And(_, rhs) = prop {
                    let prop = *rhs.clone();
                    Ok(self.add_step(prop))
                } else {
                    Err(Error::ExpectedPropVariant {
                        expected: PropVariant::And,
                        got: prop.clone(),
                    })
                }
            }
            Rule::ImplyI(proof_box) => {
                let proof_box_prop = self.get_prop(proof_box)?;
                let Prop::ProofBox {
                    assumption,
                    derived_prop,
                } = proof_box_prop
                else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box_prop.clone(),
                    });
                };

                let prop = Prop::Imply(assumption.clone(), derived_prop.clone());
                Ok(self.add_step(prop))
            }
            Rule::OrILhs(index, other) => {
                let prop = self.get_prop(index)?;
                let new = Prop::Or(Box::new(prop.clone()), Box::new(other));
                Ok(self.add_step(new))
            }
            Rule::OrIRhs(other, index) => {
                let prop = self.get_prop(index)?;
                let new = Prop::Or(Box::new(other), Box::new(prop.clone()));
                Ok(self.add_step(new))
            }
            Rule::OrE {
                or_prop,
                lhs_box,
                rhs_box,
            } => {
                let or_prop = self.get_prop(or_prop)?;
                let lhs_box = self.get_prop(lhs_box)?;
                let rhs_box = self.get_prop(rhs_box)?;

                let Prop::Or(or_lhs, or_rhs) = or_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: or_prop.clone(),
                    });
                };

                let Prop::ProofBox {
                    assumption: lhs_assumption,
                    derived_prop: lhs_derived_prop,
                } = lhs_box
                else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: lhs_box.clone(),
                    });
                };

                let Prop::ProofBox {
                    assumption: rhs_assumption,
                    derived_prop: rhs_derived_prop,
                } = lhs_box
                else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Or,
                        got: rhs_box.clone(),
                    });
                };

                check_eq(or_lhs, lhs_assumption)?;
                check_eq(or_rhs, rhs_assumption)?;
                check_eq(lhs_derived_prop, rhs_derived_prop)?;

                Ok(self.add_step(*lhs_derived_prop.clone()))
            }
            Rule::NegI(proof_box) => {
                let proof_box = self.get_prop(proof_box)?;

                let Prop::ProofBox {
                    assumption,
                    derived_prop,
                } = proof_box
                else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box.clone(),
                    });
                };

                check_eq(derived_prop, &Prop::Bottom)?;

                let negated_prop = Prop::Imply(assumption.clone(), Box::new(Prop::Bottom));
                Ok(self.add_step(negated_prop))
            }
            Rule::NegE { prop, neg_prop } => {
                let prop = self.get_prop(prop)?;
                let neg_prop = self.get_prop(neg_prop)?;

                // neg prop := prop -> bottom
                let Prop::Imply(lhs, rhs) = neg_prop else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: prop.clone(),
                    });
                };
                check_eq(lhs, prop)?;
                check_eq(rhs, &Prop::Bottom)?;

                Ok(self.add_step(Prop::Bottom))
            }
            Rule::BottomE(bottom_prop, prop) => {
                let bottom_prop = self.get_prop(bottom_prop)?;
                check_eq(bottom_prop, &Prop::Bottom)?;
                Ok(self.add_step(prop))
            }
            Rule::DoubleNegE(double_negated_prop) => {
                let double_negated_prop = self.get_prop(double_negated_prop)?;

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

                Ok(self.add_step(*prop.clone()))
            }
            Rule::ImplyE {
                implication,
                lhs_proof,
            } => {
                let implication = self.get_prop(implication)?;
                let lhs_proof = self.get_prop(lhs_proof)?;

                let Prop::Imply(lhs, rhs) = implication else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: implication.clone(),
                    });
                };

                check_eq(lhs, lhs_proof)?;

                Ok(self.add_step(*rhs.clone()))
            }
            Rule::ModusTollens {
                implication,
                negated_rhs,
            } => {
                let implication = self.get_prop(implication)?;
                let negated_rhs = self.get_prop(negated_rhs)?;

                let Prop::Imply(lhs, rhs) = implication else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: implication.clone(),
                    });
                };

                check_eq(negated_rhs, &Prop::negated(*rhs.clone()))?;

                let neg_lhs = Prop::negated(*lhs.clone());
                Ok(self.add_step(neg_lhs))
            }
            Rule::DoubleNegI(prop) => {
                let prop = self.get_prop(prop)?;
                let neg_neg_prop = Prop::negated(Prop::negated(prop.clone()));
                Ok(self.add_step(neg_neg_prop))
            }
            Rule::ProofByContradiction(proof_box) => {
                let proof_box = self.get_prop(proof_box)?;

                let Prop::ProofBox {
                    assumption,
                    derived_prop,
                } = proof_box
                else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::ProofBox,
                        got: proof_box.clone(),
                    });
                };

                // check if the assumption is negated (i.e. has the form phi -> bottom)
                let Prop::Imply(ref lhs, ref rhs) = **assumption else {
                    return Err(Error::ExpectedPropVariant {
                        expected: PropVariant::Imply,
                        got: *assumption.clone(),
                    });
                };
                check_eq(rhs, &Prop::Bottom)?;

                // also check that the proof box ends with bottom
                check_eq(derived_prop, &Prop::Bottom)?;

                Ok(self.add_step(*lhs.clone()))
            }
            Rule::LawOfExcludedMiddle(prop) => {
                let neg_prop = Prop::negated(prop.clone());
                let or_prop = Prop::Or(Box::new(prop), Box::new(neg_prop));
                Ok(self.add_step(or_prop))
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
        let mut ctx = Context::new();
        let p = ctx.add_premise(Prop::Symbol("p".into()));
        let q = ctx.add_premise(Prop::Symbol("q".into()));
        let p_and_q = ctx.apply_rule(Rule::AndI(p, q)).unwrap();

        assert_eq!(
            ctx.get_prop(p_and_q).unwrap(),
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

        let mut ctx = Context::new();
        let q = ctx.add_premise(Prop::Symbol("q".into()));
        let p = ctx.add_assumption(Prop::Symbol("p".into()));
        let _ = ctx.copy(q).unwrap();
        ctx.close_scope().unwrap();
        let p_implies_q = ctx.apply_rule(Rule::ImplyI(p)).unwrap();

        assert_eq!(
            ctx.get_prop(p_implies_q).unwrap(),
            &Prop::Imply(
                Box::new(Prop::Symbol("p".into())),
                Box::new(Prop::Symbol("q".into())),
            )
        );
    }
}
