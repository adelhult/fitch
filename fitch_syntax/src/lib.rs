use chumsky::{prelude::*, text::whitespace};
use fitch_core::{Prop, Rule, StepIndex};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Rule(Rule), // TODO: Rule
    Copy(StepIndex),
    Premise(Prop),
    Assume(Prop),
    Discharge,
    Quit,
    Help, // TODO: add an Option<String> to get help about a specific rule
          // TODO: Revert, LaTeX, Table
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Rule(rule) => write!(f, "{rule}"),
            Command::Copy(i) => write!(f, "copy {i}"),
            Command::Premise(_) => write!(f, "premise"),
            Command::Assume(_) => write!(f, "assumption"),
            Command::Discharge => write!(f, "discharged assumption"),
            Command::Quit => write!(f, "quit"),
            Command::Help => write!(f, "help"),
        }
    }
}

pub fn parse_command(s: &str) -> Result<Command, Vec<chumsky::error::Simple<char>>> {
    command().parse(s)
}

fn command() -> impl Parser<char, Command, Error = Simple<char>> {
    let rule = just("rule")
        .ignore_then(whitespace())
        .ignore_then(rule())
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
        .map(Command::Assume);

    let discharge = just("discharge").map(|_| Command::Discharge);
    let quit = just("quit").or(just("exit")).map(|_| Command::Quit);
    let help = just("help").map(|_| Command::Help);

    choice((rule, copy, premise, assume, discharge, quit, help))
        .labelled("command")
        .padded()
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

fn rule() -> impl Parser<char, Rule, Error = Simple<char>> {
    let and_prefix = just("and_").or(just("∧")).or(just("&")).or(just("^"));

    let and_i = and_prefix
        .then(just("i"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(index())
        .map(|(lhs, rhs)| Rule::AndI(lhs, rhs));

    let and_e_lhs = and_prefix
        .then(just("e_lhs"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::AndELhs);

    let and_e_rhs = and_prefix
        .then(just("e_rhs"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::AndERhs);

    let or_prefix = just("or_").or(just("∨")).or(just("|")).or(just("v"));

    let or_i_lhs = or_prefix
        .then(just("i_lhs"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(prop())
        .map(|(lhs, rhs)| Rule::OrILhs(lhs, rhs));

    let or_i_rhs = or_prefix
        .then(just("i_rhs"))
        .ignore_then(whitespace())
        .ignore_then(prop())
        .then_ignore(whitespace())
        .then(index())
        .map(|(lhs, rhs)| Rule::OrIRhs(lhs, rhs));

    let or_e = or_prefix
        .then(just("e"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(index())
        .then_ignore(whitespace())
        .then(index())
        .map(|((or_prop, lhs_box), rhs_box)| Rule::OrE {
            or_prop,
            lhs_box,
            rhs_box,
        });

    let neg_prefix = just("neg_").or(just("-")).or(just("¬"));

    let neg_i = neg_prefix
        .then(just("i"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::NegI);

    let neg_e = neg_prefix
        .then(just("e"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(index())
        .map(|(prop, neg_prop)| Rule::NegE { prop, neg_prop });

    let imply_prefix = just("imply_").or(just("->")).or(just("⇒")).or(just("→"));

    let imply_i = imply_prefix
        .then(just("i"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::ImplyI);

    let imply_e = imply_prefix
        .then(just("e"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(index())
        .map(|(implication, lhs_proof)| Rule::ImplyE {
            implication,
            lhs_proof,
        });

    let bottom_e = just("bottom_e")
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(prop())
        .map(|(bottom, prop)| Rule::BottomE(bottom, prop));

    let double_neg_prefix = just("neg_neg_").or(just("--")).or(just("¬¬"));

    let double_neg_e = double_neg_prefix
        .then(just("e"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::DoubleNegE);

    let double_neg_i = double_neg_prefix
        .then(just("i"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::DoubleNegI);

    let modus_tollens = just("modus_tollens")
        .or(just("mt"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .then_ignore(whitespace())
        .then(index())
        .map(|(implication, negated_rhs)| Rule::ModusTollens {
            implication,
            negated_rhs,
        });

    let proof_by_contradiction = just("pbc")
        .or(just("proof_by_contradiction"))
        .ignore_then(whitespace())
        .ignore_then(index())
        .map(Rule::ProofByContradiction);

    let law_of_excluded_middle = just("lem")
        .or(just("law_of_excluded_middle"))
        .ignore_then(whitespace())
        .ignore_then(prop())
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
