use fitch_core::{Context, Error};
use fitch_syntax::{parse_command, Command};
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();

    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("fitch".into()),
        DefaultPromptSegment::Empty,
    );

    let mut ctx = Context::new();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => match parse_command(&buffer) {
                Ok(command) => match run(command, &mut ctx) {
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

fn run(command: Command, ctx: &mut Context) -> Result<bool, Error> {
    let (should_continue, result) = match command.clone() {
        Command::Rule(rule) => (true, Some(ctx.apply_rule(&rule)?)),
        Command::Copy(i) => (true, Some(ctx.copy(i)?)),
        Command::Premise(prop) => (true, Some(ctx.add_premise(prop))),
        Command::Assume(prop) => (true, Some(ctx.add_assumption(prop))),
        Command::Discharge => {
            ctx.close_scope()?;
            (true, None)
        }
        Command::Quit => (false, None),
        Command::Help => (true, None),
    };

    if let Some((index, prop)) = result {
        println!(
            "{}",
            format!(
                "{index: >5} | {prop} {command: >40}",
                prop = prop,
                index = index.to_string(),
                command = command.to_string()
            )
        );
    }

    Ok(should_continue)
}
