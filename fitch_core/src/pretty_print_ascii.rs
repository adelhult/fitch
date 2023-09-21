use crate::{Proof, Prop, Step, StepIndex, SubProof};

pub fn print_proof(proof: &Proof) {
    for (level, scope) in proof.context.iter().enumerate() {
        let mut steps = scope.steps.iter().collect::<Vec<_>>();
        steps.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));
        println!("{}", steps_to_string(steps.as_slice(), level));
    }
}

// TODO: use write! macro instead of allocating new strings for each recursive call
fn steps_to_string(steps: &[(&StepIndex, &Step)], indent: usize) -> String {
    let mut s = String::new();
    for (i, step) in steps {
        match step.prop() {
            Prop::ProofBox(SubProof(subproof)) => s.push_str(&steps_to_string(
                subproof
                    .iter()
                    .map(|(i, step)| (i, step))
                    .collect::<Vec<_>>()
                    .as_slice(),
                indent + 1,
            )),
            // prop with indentioni: >4
            _ => s.push_str(&format!(
                "{i: >3} {prop_with_indent: <40} {step_type}\n",
                i = i.to_string(),
                step_type = step.step_type(),
                prop_with_indent = prop_with_indent(step.prop(), indent),
            )),
        }
    }
    s
}

fn prop_with_indent(prop: &Prop, indent: usize) -> String {
    let mut s = String::new();
    for _ in 0..indent {
        s.push_str("  ");
    }
    s.push_str(" | ");
    s.push_str(&prop.to_string());
    s
}
