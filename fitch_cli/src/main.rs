use colored::*;
use fitch_core::{latex, print_proof, Error, Proof};
use fitch_syntax::{parse_command, Command, Source};
use rand::seq::SliceRandom;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn greet() {
    println!(
        r#"{greeting}
A command-line editor for natural deduction
proofs (for propositional logic).
    
{intro}
"#,
        greeting = "Hi! I'm Fitch.".bold(),
        intro = "Get started by typing a command, for example:
premise p & q
rule &e 1
copy 5
assume (p | q) -> -q 
discharge
latex
help
quit"
            .italic()
    );
}

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

    greet();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(line)) => match parse_command(&line) {
                Ok(command) => match run(command, &mut proof, &mut line_editor) {
                    Ok(false) => break,
                    Ok(true) => continue,
                    Err(error) => {
                        println!("Error: {}", error);
                        continue;
                    }
                },
                Err(errors) => errors
                    .into_iter()
                    .for_each(|report| report.eprint(Source::from(&line)).unwrap()),
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
        Command::Latex => {
            if let Some(latex_text) = latex(&proof) {
                println!("{imports}{latex_text}", imports = "Remember to also include these packages:\n\\usepackage{amsmath}\n\\usepackage{logicproof}\n\n".bright_black());
            } else {
                println!(
                    "Could not typeset this proof. Maybe you have not closed all your proof boxes?"
                );
            }
            (true, false)
        }
    };

    if clear_screen {
        line_editor.clear_screen().unwrap();
        print_proof(proof);
    }

    Ok(should_continue)
}
