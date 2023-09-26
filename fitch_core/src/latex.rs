use crate::{Proof, Prop, Rule, Step, StepIndex, StepType, SubProof};
use std::fmt::Write;

pub fn latex(proof: &Proof) -> Option<String> {
    if proof.context.len() > 1 {
        // You may not have any open proof boxes when generating latex output
        return None;
    }
    let scope = proof.context.get(0)?;
    let mut steps = scope.steps.iter().collect::<Vec<_>>();
    steps.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

    let mut result = String::new();

    // The logicproof environment needs to now the max nesting of subproofs, so we need to
    // make first pass to find the max depth
    let max_depth = max_depth(
        steps
            .iter()
            .map(|(_, step)| *step)
            .collect::<Vec<_>>()
            .as_slice(),
    );

    write!(result, "\\begin{{logicproof}}{{{max_depth}}}\n").unwrap();
    steps_to_string(&mut result, steps.as_slice(), 0);
    write!(result, "\\end{{logicproof}}\n").unwrap();
    Some(result)
}

fn steps_to_string(s: &mut String, steps: &[(&StepIndex, &Step)], indent_level: usize) {
    // Add a new sub-proof if we are indented
    if indent_level > 0 {
        write!(s, "\\begin{{subproof}}\n").unwrap();
    }

    // Print all the steps (any possibly recursively print completed sub-proofs)
    for (i, (_, step)) in steps.iter().enumerate() {
        match step.prop() {
            Prop::ProofBox(SubProof(subproof)) => {
                steps_to_string(
                    s,
                    subproof
                        .iter()
                        .map(|(i, step)| (i, step))
                        .collect::<Vec<_>>()
                        .as_slice(),
                    indent_level + 1,
                );
            }

            _ => s.push_str(&format!(
                "{prop} & {step_type}{newline}\n",
                step_type = latex_step_type(step.step_type()),
                prop = latex_prop(step.prop()),
                newline = if i == steps.len() - 1 { "" } else { " \\\\" }
            )),
        }
    }

    if indent_level > 0 {
        write!(s, "\\end{{subproof}}\n").unwrap();
    }
}

fn max_depth(steps: &[&Step]) -> usize {
    steps
        .iter()
        .map(|step| match step.prop() {
            Prop::ProofBox(SubProof(steps)) => {
                1 + max_depth(
                    steps
                        .iter()
                        .map(|(_, step)| step)
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
            }
            _ => 0,
        })
        .max()
        .unwrap_or(0)
}

fn latex_prop(prop: &Prop) -> String {
    use Prop::*;
    match prop {
        Bottom => r"\bot".to_string(),
        Symbol(symbol) => format!(r"\text{{{symbol}}}"),
        And(phi, psi) => format!(
            r"{phi} \land {psi}",
            phi = latex_prop(phi),
            psi = latex_prop(psi)
        ),
        Or(phi, psi) => format!(
            r"{phi} \lor {psi}",
            phi = latex_prop(phi),
            psi = latex_prop(psi)
        ),
        Imply(phi, psi) => format!(
            r"{phi} \to {psi}",
            phi = latex_prop(phi),
            psi = latex_prop(psi)
        ),
        ProofBox(_) => "sub-proof".to_string(),
    }
}

fn latex_step_type(step_type: &StepType) -> String {
    use StepType::*;
    match step_type {
        Rule(rule) => latex_rule(rule),
        Copy(i) => format!("copy {i}"),
        Premise => "premise".to_string(),
        Assumption => "assumption".to_string(),
    }
}

fn latex_rule(rule: &Rule) -> String {
    use Rule::*;
    match rule {
        AndI(i, j) => format!(r"$\land_{{I}}$ {i}, {j}"),
        AndELhs(i) => format!(r"$\land_{{E_{{LHS}}}}$ {i}"),
        AndERhs(i) => format!(r"$\land_{{E_{{RHS}}}}$ {i}"),
        OrILhs(i, prop) => format!(r"$\lor_{{I_{{LHS}}}}$ {i}, {prop}", prop = latex_prop(prop)),
        OrIRhs(prop, j) => format!(r"$\lor_{{I_{{RHS}}}}$ {prop}, {j}", prop = latex_prop(prop)),
        OrE {
            or_prop,
            lhs_box,
            rhs_box,
        } => format!(r"$\lor_E$ {or_prop}, {lhs_box}, {rhs_box}"),
        NegI(i) => format!(r"$\neg_I$ {i}"),
        NegE { prop, neg_prop } => format!(r"$\neg_E$ {prop} {neg_prop}"),
        ImplyI(i) => format!(r"$\to_I$ {i}"),
        ImplyE {
            implication,
            lhs_proof,
        } => format!(r"$\to_E$ {implication}, {lhs_proof}"),
        BottomE(i, prop) => format!(r"$\bot_E$ {i}, {prop}", prop = latex_prop(prop)),
        DoubleNegE(i) => format!(r"$\neg\neg_E$ {i}"),
        ModusTollens {
            implication,
            negated_rhs,
        } => format!(r"MT {implication} {negated_rhs}"),
        DoubleNegI(i) => format!(r"$\neg\neg_I$ {i}"),
        ProofByContradiction(i) => format!(r"PBC {i}"),
        LawOfExcludedMiddle(i) => format!(r"LEM {i}"),
    }
}
