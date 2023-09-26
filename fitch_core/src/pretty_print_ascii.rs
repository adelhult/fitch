use crate::{Proof, Prop, Step, StepIndex, SubProof};

// TODO: I should really replace all the hard-coded magic numbers with proper constants.
const WIDTH: usize = 70;

pub fn print_proof(proof: &Proof) {
    for (level, scope) in proof.context.iter().enumerate() {
        let mut steps = scope.steps.iter().collect::<Vec<_>>();
        steps.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        print!("{}", steps_to_string(steps.as_slice(), level, false));
    }
}

fn steps_to_string(steps: &[(&StepIndex, &Step)], indent_level: usize, closed: bool) -> String {
    let mut s = String::new();

    // Add a line if we are inside of a subproof
    if indent_level > 0 {
        let line = format!(
            "    {indent}┌{hline}┐{indent}\n",
            indent = if indent_level > 0 { "│" } else { "" }.repeat(indent_level - 1),
            hline = "─".repeat(WIDTH - indent_level * 2 - 7)
        );
        s.push_str(&line);
    }

    // Print all the steps (any possibly recursivly print completed subproofs)
    for (i, step) in steps {
        match step.prop() {
            Prop::ProofBox(SubProof(subproof)) => {
                s.push_str(&steps_to_string(
                    subproof
                        .iter()
                        .map(|(i, step)| (i, step))
                        .collect::<Vec<_>>()
                        .as_slice(),
                    indent_level + 1,
                    true,
                ));
            }

            _ => s.push_str(&format!(
                "{index: >3} {indent} {prop: <prop_width$} {step_type: >20} {indent}\n",
                index = i.to_string(),
                step_type = step.step_type().to_string(),
                indent = if indent_level > 0 { "│" } else { "" }.repeat(indent_level),
                prop = step.prop().to_string(),
                prop_width = 40 - indent_level * 2,
            )),
        }
    }

    if closed && indent_level > 0 {
        let line = format!(
            "    {indent}└{hline}┘{indent}\n",
            indent = if indent_level > 0 { "│" } else { "" }.repeat(indent_level - 1),
            hline = "─".repeat(WIDTH - indent_level * 2 - 7)
        );
        s.push_str(&line);
    }

    s
}
