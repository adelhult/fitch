use chumsky::{prelude::*, text::whitespace};
use fitch_core::{Prop, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Rule(String), // TODO: Rule
    Copy(usize),
    Premise(String), // TODO Prop
    Assume(String),  // TODO Prop
    Discharge,
    Quit,
    // TODO: Revert, LaTeX, Exit
}

pub fn parse_command(s: &str) -> Command {
    command().parse(s).unwrap()
}

fn index() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10).map(|n: String| n.parse().unwrap())
}

fn command() -> impl Parser<char, Command, Error = Simple<char>> {
    let rule = text::keyword("rule")
        .ignore_then(whitespace())
        .ignore_then(text::ident()) // TODO: Add a rule parser
        .map(Command::Rule);

    let copy = text::keyword("copy")
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Command::Copy);

    let premise = text::keyword("premise")
        .ignore_then(whitespace())
        .ignore_then(text::ident()) // TODO: Add a prop parser
        .map(Command::Premise);

    let assume = text::keyword("assume")
        .ignore_then(whitespace())
        .ignore_then(text::ident()) // TODO: Add a prop parser
        .map(Command::Premise);

    let discharge = text::keyword("discharge").map(|_| Command::Discharge);

    let quit = text::keyword("quit")
        .or(text::keyword("exit"))
        .map(|_| Command::Quit);

    rule.or(copy)
        .or(premise)
        .or(assume)
        .or(discharge)
        .or(quit)
        .padded()
}
