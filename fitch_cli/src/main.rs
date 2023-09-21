use fitch_core::{print_proof, Error, Proof};
use fitch_syntax::{parse_command, Command};
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();

    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("fitch".into()),
        DefaultPromptSegment::Empty,
    );

    let mut proof = Proof::new();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => match parse_command(&buffer) {
                Ok(command) => match run(command, &mut proof) {
                    Ok(false) => break,
                    Ok(true) => continue,
                    Err(error) => {
                        println!("Error: {}", error);
                        continue;
                    }
                },
                Err(errors) => {
                    if let Some(error) = errors.first() {
                        println!(
                            "Error when trying to parse {}.",
                            error.label().unwrap_or_else(|| "command")
                        )
                    }
                }
            },
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
}

fn run(command: Command, proof: &mut Proof) -> Result<bool, Error> {
    let (should_continue, result) = match command.clone() {
        Command::Rule(rule) => (true, Some(proof.apply_rule(&rule)?)),
        Command::Copy(i) => (true, Some(proof.copy(i)?)),
        Command::Premise(prop) => (true, Some(proof.add_premise(prop))),
        Command::Assume(prop) => (true, Some(proof.add_assumption(prop))),
        Command::Discharge => {
            proof.close_scope()?;
            (true, None)
        }
        Command::Quit => (false, None),
        Command::Help => (true, None),
    };

    if let Some(_) = result {
        print_proof(proof);
    }

    Ok(should_continue)
}
