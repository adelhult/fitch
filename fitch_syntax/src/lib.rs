use chumsky::{prelude::*, text::whitespace};
use fitch_core::{Prop, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Rule(String), // TODO: Rule
    Copy(usize),
    Premise(Prop),
    Assume(Prop),
    Discharge,
    Quit,
    // TODO: Revert, LaTeX, Exit
}

pub fn parse_command(s: &str) -> Command {
    command().parse(s).unwrap()
}

fn command() -> impl Parser<char, Command, Error = Simple<char>> {
    let rule = just("rule")
        .ignore_then(whitespace())
        .ignore_then(text::ident()) // TODO: Add a rule parser
        .map(Command::Rule);

    let copy = just("copy")
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Command::Copy);

    let premise = just("premise")
        .ignore_then(whitespace())
        .ignore_then(prop())
        .map(Command::Premise);

    let assume = just("assume")
        .ignore_then(whitespace())
        .ignore_then(prop())
        .map(Command::Premise);

    let discharge = just("discharge").map(|_| Command::Discharge);

    let quit = just("quit").or(just("exit")).map(|_| Command::Quit);

    rule.or(copy)
        .or(premise)
        .or(assume)
        .or(discharge)
        .or(quit)
        .padded()
        .then_ignore(end())
}

fn index() -> impl Parser<char, usize, Error = Simple<char>> {
    text::int(10).map(|n: String| n.parse().unwrap())
}

fn prop() -> impl Parser<char, Prop, Error = Simple<char>> {
    recursive(|prop| {
        let bottom = just("bottom").or(just("⊥")).map(|_| Prop::Bottom);
        let symbol = text::ident().map(Prop::Symbol);

        let atom = bottom
            .or(symbol)
            .or(prop.delimited_by(just('('), just(')')));

        let negate_op = just('-').or(just('¬')).padded();
        let negate = negate_op
            .repeated()
            .then(atom)
            // Note negation is implemented as -phi = phi -> bottom
            .foldr(|_op, rhs| Prop::Imply(Box::new(rhs), Box::new(Prop::Bottom)));

        let and_op = just('*').or(just('∧')).or(just('&')).padded();
        let and = negate
            .clone()
            .then(and_op.ignore_then(negate).repeated())
            .foldl(|lhs, rhs| Prop::And(Box::new(lhs), Box::new(rhs)));

        let or_op = just('+').or(just('∨')).or(just('|').or(just('v'))).padded();
        let or = and
            .clone()
            .then(or_op.ignore_then(and).repeated())
            .foldl(|lhs, rhs| Prop::Or(Box::new(lhs), Box::new(rhs)));

        let implies_op = just("->").or(just("⇒")).or(just("→")).padded();
        let implies = or
            .clone()
            .then_ignore(implies_op)
            .repeated()
            .then(or)
            .foldr(|lhs, rhs| Prop::Imply(Box::new(lhs), Box::new(rhs)));

        implies
    })
    // TODO: Maybe add a .then_ignore(end())
}
