use fitch_core::{print_proof, Error, Proof};
use fitch_syntax::{parse_command, Command};
use rand::seq::SliceRandom;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

const WELCOME: &str = r#"Hi! I'm Fitch, a command-line editor
for natural deduction proofs (with propositional logic).

Get started by typing a command, for example:
premise p & q
rule &e 1
copy 5
assume (p | q) -> -q 
discharge
quit
help
"#;

fn say_goodbye() {
    let phrase = [
        "Bye!",
        "See you later!",
        "Have a nice day!",
        "See you soon!",
        "See you next time!",
    ];
    let mut rng = rand::thread_rng();
    let goodbye = phrase.choose(&mut rng).unwrap();
    println!("{goodbye}");
}

fn main() {
    let mut line_editor = Reedline::create();

    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("fitch".into()),
        DefaultPromptSegment::Empty,
    );

    let mut proof = Proof::new();

    println!("{WELCOME}");

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => match parse_command(&buffer) {
                Ok(command) => match run(command, &mut proof, &mut line_editor) {
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
                say_goodbye();
                break;
            }
            // TODO: Add CTRL+Z
            _ => (),
        }
    }
}

fn run(command: Command, proof: &mut Proof, line_editor: &mut Reedline) -> Result<bool, Error> {
    let (should_continue, clear_screen) = match command.clone() {
        // TODO: remove the result from the tuple
        Command::Rule(rule) => {
            proof.apply_rule(&rule)?;
            (true, true)
        }
        Command::Copy(i) => {
            proof.copy(i)?;
            (true, true)
        }
        Command::Premise(prop) => {
            proof.add_premise(prop);
            (true, true)
        }
        Command::Assume(prop) => {
            proof.add_assumption(prop);
            (true, true)
        }
        Command::Discharge => {
            proof.close_scope()?;
            (true, true)
        }
        Command::Quit => {
            say_goodbye();
            (false, false)
        }
        Command::Help => {
            println!("TODO: add help. You are own your own for now :^)");
            (true, false)
        }
    };

    if clear_screen {
        line_editor.clear_screen().unwrap();
        print_proof(proof);
    }

    Ok(should_continue)
}
