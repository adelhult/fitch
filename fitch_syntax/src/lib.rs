use std::fmt;

use ariadne::{Color, Fmt, Label, Report, ReportKind};
use chumsky::{error::SimpleReason, prelude::*, Stream};
use fitch_core::{Prop, Rule, RuleName, StepIndex};

pub type Span = std::ops::Range<usize>;
pub use ariadne::Source;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Rule(Rule),
    Copy(StepIndex),
    Premise(Prop),
    Assume(Prop),
    Discharge,
    Quit,
    Help, // TODO: add an Option<String> to get help about a specific rule
          // TODO: Revert, LaTeX, Table
}

pub fn parse_command(s: &str) -> Result<Command, Vec<Report<'_>>> {
    let tokens = lexer().parse(s).map_err(|errors| {
        errors
            .into_iter()
            .map(|error| error.map(|c| c.to_string()))
            .map(generate_report)
            .collect::<Vec<_>>()
    })?;

    let len = s.chars().count();
    let command = command()
        .parse(Stream::from_iter(len..len + 1, tokens.into_iter()))
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|error| error.map(|c| c.to_string()))
                .map(generate_report)
                .collect::<Vec<_>>()
        })?;

    Ok(command)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    Rule,
    Copy,
    Premise,
    Assume,
    Discharge,
    Quit,
    Help,
    Index(StepIndex),
    Prop(Prop),
    RuleName(RuleName),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Rule => write!(f, "rule"),
            Token::Copy => write!(f, "copy"),
            Token::Premise => write!(f, "premise"),
            Token::Assume => write!(f, "assume"),
            Token::Discharge => write!(f, "discharge"),
            Token::Quit => write!(f, "quit"),
            Token::Help => write!(f, "help"),
            Token::Index(i) => write!(f, "{i}"),
            Token::Prop(prop) => write!(f, "{prop}"),
            Token::RuleName(name) => write!(f, "{name}"),
        }
    }
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    choice((
        just("rule").map(|_| Token::Rule),
        just("copy").map(|_| Token::Copy),
        just("premise").map(|_| Token::Premise),
        just("assume").map(|_| Token::Assume),
        just("discharge").map(|_| Token::Discharge),
        just("quit").map(|_| Token::Quit),
        just("help").map(|_| Token::Help),
        index().map(Token::Index),
        prop().map(Token::Prop),
        rule_name().map(Token::RuleName),
    ))
    .map_with_span(|token, span| (token, span))
    .padded()
    .repeated()
    // TODO: Not totally sure this is a good idea, the nano_rust example from chumsky does not do this
    .then_ignore(end())
    .collect()
}

fn command() -> impl Parser<Token, Command, Error = Simple<Token>> {
    let prop = select! {Token::Prop(prop) => prop};
    let index = select! {Token::Index(i) => i};

    let copy = just(Token::Copy).ignore_then(index).map(Command::Copy);
    let premise = just(Token::Premise).ignore_then(prop).map(Command::Premise);
    let assume = just(Token::Assume).ignore_then(prop).map(Command::Assume);
    let rule = just(Token::Rule).ignore_then(rule()).map(Command::Rule);

    choice((
        copy,
        premise,
        assume,
        rule,
        select! {
            Token::Discharge => Command::Discharge,
            Token::Quit => Command::Quit,
            Token::Help => Command::Help,
        },
    ))
    .labelled("command")
    .then_ignore(end())
}

fn index() -> impl Parser<char, StepIndex, Error = Simple<char>> {
    text::int(10)
        .map(|n: String| n.parse().unwrap())
        .map(StepIndex)
        .labelled("index")
}

fn prop() -> impl Parser<char, Prop, Error = Simple<char>> {
    recursive(|prop| {
        let bottom = just("bottom").or(just("⊥")).map(|_| Prop::Bottom);
        let symbol = text::ident().map(Prop::Symbol);

        let atom = bottom
            .or(symbol)
            .or(prop.delimited_by(just('('), just(')')));
        // TODO: Maybe use chumsky::recovery::nested_delimiters

        let negate_op = just('-').or(just('¬')).padded();
        let negate = negate_op
            .repeated()
            .then(atom)
            // Note negation is implemented as -phi = phi -> bottom
            .foldr(|_op, rhs| Prop::Imply(Box::new(rhs), Box::new(Prop::Bottom)));

        let and_op = just('*').or(just('∧')).or(just('&')).or(just('^')).padded();
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
}

fn rule_name() -> impl Parser<char, RuleName, Error = Simple<char>> {
    let and_prefix = just("and_").or(just("∧")).or(just("&")).or(just("^"));
    let or_prefix = just("or_").or(just("∨")).or(just("|")).or(just("v"));
    let neg_prefix = just("neg_").or(just("-")).or(just("¬"));
    let imply_prefix = just("imply_").or(just("->")).or(just("⇒")).or(just("→"));
    let double_neg_prefix = just("neg_neg_").or(just("--")).or(just("¬¬"));

    let and_i = and_prefix.then_ignore(just("i")).to(RuleName::AndI);
    let and_e_lhs = and_prefix.then_ignore(just("e_lhs")).to(RuleName::AndELhs);
    let and_e_rhs = and_prefix.then_ignore(just("e_rhs")).to(RuleName::AndERhs);
    let or_i_lhs = or_prefix.then_ignore(just("i_lhs")).to(RuleName::OrILhs);
    let or_i_rhs = or_prefix.then_ignore(just("i_rhs")).to(RuleName::OrIRhs);
    let or_e = or_prefix.then_ignore(just("e")).to(RuleName::OrE);
    let neg_i = neg_prefix.then_ignore(just("i")).to(RuleName::NegI);
    let neg_e = neg_prefix.then_ignore(just("e")).to(RuleName::NegE);
    let imply_i = imply_prefix.then_ignore(just("i")).to(RuleName::ImplyI);
    let imply_e = imply_prefix.then_ignore(just("e")).to(RuleName::ImplyE);
    let bottom_e = just("bottom_e").to(RuleName::BottomE);
    let double_neg_e = double_neg_prefix
        .then_ignore(just("e"))
        .to(RuleName::DoubleNegE);
    let double_neg_i = double_neg_prefix
        .then_ignore(just("i"))
        .to(RuleName::DoubleNegI);
    let modus_tollens = just("modus_tollens")
        .or(just("mt"))
        .to(RuleName::ModusTollens);
    let proof_by_contradiction = just("pbc")
        .or(just("proof_by_contradiction"))
        .to(RuleName::ProofByContradiction);
    let law_of_excluded_middle = just("lem")
        .or(just("law_of_excluded_middle"))
        .to(RuleName::LawOfExcludedMiddle);

    choice((
        and_i,
        and_e_lhs,
        and_e_rhs,
        or_i_lhs,
        or_i_rhs,
        or_e,
        neg_i,
        neg_e,
        imply_i,
        imply_e,
        bottom_e,
        double_neg_e,
        double_neg_i,
        modus_tollens,
        proof_by_contradiction,
        law_of_excluded_middle,
    ))
    .labelled("rule name")
}

fn rule() -> impl Parser<Token, Rule, Error = Simple<Token>> {
    let index = select! { Token::Index(i) => i};
    let prop = select! { Token::Prop(p) => p};

    let and_i = select! {Token::RuleName(RuleName::AndI) => ()}
        .ignore_then(index)
        .then(index)
        .map(|(lhs, rhs)| Rule::AndI(lhs, rhs));

    let and_e_lhs = select! {Token::RuleName(RuleName::AndELhs) => ()}
        .ignore_then(index)
        .map(Rule::AndELhs);

    let and_e_rhs = select! {Token::RuleName(RuleName::AndERhs) => ()}
        .ignore_then(index)
        .map(Rule::AndERhs);

    let or_i_lhs = select! {Token::RuleName(RuleName::OrILhs) => ()}
        .ignore_then(index)
        .then(prop)
        .map(|(lhs, rhs)| Rule::OrILhs(lhs, rhs));

    let or_i_rhs = select! {Token::RuleName(RuleName::OrIRhs) => ()}
        .ignore_then(prop)
        .then(index)
        .map(|(lhs, rhs)| Rule::OrIRhs(lhs, rhs));

    let or_e = select! {Token::RuleName(RuleName::OrE) => ()}
        .ignore_then(index)
        .then(index)
        .then(index)
        .map(|((or_prop, lhs_box), rhs_box)| Rule::OrE {
            or_prop,
            lhs_box,
            rhs_box,
        });

    let neg_i = select! {Token::RuleName(RuleName::NegI) => ()}
        .ignore_then(index)
        .map(Rule::NegI);

    let neg_e = select! {Token::RuleName(RuleName::NegE) => ()}
        .ignore_then(index)
        .then(index)
        .map(|(prop, neg_prop)| Rule::NegE { prop, neg_prop });

    let imply_i = select! {Token::RuleName(RuleName::ImplyI) => ()}
        .ignore_then(index)
        .map(Rule::ImplyI);

    let imply_e = select! {Token::RuleName(RuleName::ImplyE) => ()}
        .ignore_then(index)
        .then(index)
        .map(|(implication, lhs_proof)| Rule::ImplyE {
            implication,
            lhs_proof,
        });

    let bottom_e = select! {Token::RuleName(RuleName::BottomE) => ()}
        .ignore_then(index)
        .then(prop)
        .map(|(bottom, prop)| Rule::BottomE(bottom, prop));

    let double_neg_e = select! {Token::RuleName(RuleName::DoubleNegE) => ()}
        .ignore_then(index)
        .map(Rule::DoubleNegE);

    let double_neg_i = select! {Token::RuleName(RuleName::DoubleNegI) => ()}
        .ignore_then(index)
        .map(Rule::DoubleNegI);

    let modus_tollens = select! {Token::RuleName(RuleName::ModusTollens) => ()}
        .ignore_then(index)
        .then(index)
        .map(|(implication, negated_rhs)| Rule::ModusTollens {
            implication,
            negated_rhs,
        });

    let proof_by_contradiction = select! {Token::RuleName(RuleName::ProofByContradiction) => ()}
        .ignore_then(index)
        .map(Rule::ProofByContradiction);

    let law_of_excluded_middle = select! {Token::RuleName(RuleName::LawOfExcludedMiddle) => ()}
        .ignore_then(prop)
        .map(Rule::LawOfExcludedMiddle);

    choice((
        and_i,
        and_e_lhs,
        and_e_rhs,
        or_i_lhs,
        or_i_rhs,
        or_e,
        neg_i,
        neg_e,
        imply_i,
        imply_e,
        bottom_e,
        double_neg_e,
        double_neg_i,
        modus_tollens,
        proof_by_contradiction,
        law_of_excluded_middle,
    ))
    .labelled("rule")
}

fn generate_report(error: Simple<String>) -> Report<'static> {
    let report = Report::build(ReportKind::Error, (), error.span().start);
    let report = match error.reason() {
        SimpleReason::Unclosed { span, delimiter } => report
            .with_message(format!(
                "Unclosed delimiter {}",
                delimiter.fg(Color::Yellow)
            ))
            .with_label(
                Label::new(span.clone())
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_color(Color::Yellow),
            )
            .with_label(
                Label::new(error.span())
                    .with_message(format!(
                        "Must be closed before this {}",
                        error
                            .found()
                            .unwrap_or(&"end of file".to_string())
                            .fg(Color::Red)
                    ))
                    .with_color(Color::Red),
            ),
        SimpleReason::Unexpected => report
            .with_message(format!(
                "{}, expected {}",
                if error.found().is_some() {
                    "Unexpected token in input"
                } else {
                    "Unexpected end of input"
                },
                if error.expected().len() == 0 {
                    "something else".to_string()
                } else {
                    error
                        .expected()
                        .map(|expected| match expected {
                            Some(expected) => expected.to_string(),
                            None => "end of input".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ))
            .with_label(
                Label::new(error.span())
                    .with_message(format!(
                        "Unexpected token {}",
                        error
                            .found()
                            .unwrap_or(&"end of file".to_string())
                            .fg(Color::Red)
                    ))
                    .with_color(Color::Red),
            ),
        SimpleReason::Custom(msg) => report.with_message(msg).with_label(
            Label::new(error.span())
                .with_message(format!("{}", msg.fg(Color::Red)))
                .with_color(Color::Red),
        ),
    };
    report.finish()
}
