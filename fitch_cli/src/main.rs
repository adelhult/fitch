use fitch_core::Context;
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
                Ok(command) => {
                    if !run(command, &mut ctx) {
                        break;
                    }
                }
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

fn run(command: Command, ctx: &mut Context) -> bool {
    let should_continue = match command {
        Command::Rule(rule) => {
            let result = ctx.apply_rule(rule);
            println!("{:?}", result);
            true
        }
        Command::Copy(i) => {
            let result = ctx.copy(i);
            println!("{:?}", result);
            true
        }
        Command::Premise(prop) => {
            let result = ctx.add_premise(prop);
            println!("{:?}", result);
            true
        }
        Command::Assume(prop) => {
            let result = ctx.add_assumption(prop);
            println!("{:?}", result);
            true
        }
        Command::Discharge => {
            let result = ctx.close_scope();
            println!("{:?}", result);
            true
        }
        Command::Quit => false,
        Command::Help => {
            println!("TODO: implement help menu");
            true
        }
    };

    //println!("{:#?}", ctx);

    should_continue
}
