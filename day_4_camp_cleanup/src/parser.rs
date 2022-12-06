#![allow(clippy::result_large_err)]

use pear::{
    combinators::*,
    input::{Pear, Text},
    macros::{parse_error, parser},
    parse,
    parsers::*,
};

use crate::{Assignment, Pair, Section};

/// Use `Text` as the `Input`; this parses `char` tokens.
type Input<'a> = Pear<Text<'a>>;

/// See [`pear::macros::parser`] for details.
type Result<'a, T> = pear::input::Result<T, Input<'a>>;

/// Returns true if the char is a digit char (0, 1, 2, ..., 9)
#[inline]
fn is_num_char(&byte: &char) -> bool {
    matches!(byte, '0'..='9')
}

/// Parse section ID from numeric characters.
#[parser]
fn section<'a>(input: &mut Input<'a>) -> Result<'a, Section> {
    take_some_while(is_num_char)?
        .parse()
        .or_else(|e| parse_error!("{}", e)?)
}

/// Parse assigned section IDs.
#[parser]
fn assignment<'a>(input: &mut Input<'a>) -> Result<'a, Assignment> {
    let start = section()?;
    eat('-')?;
    let end = section()?;
    Assignment::new(start, end)
}

/// Parse pair of assignments.
#[parser]
fn pair<'a>(input: &mut Input<'a>) -> Result<'a, Pair> {
    let first = assignment()?;
    eat(',')?;
    let second = assignment()?;
    eat('\n')?;
    Pair(first, second)
}

/// Parse list of pair of assignments.
#[parser]
fn pairs<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Pair>> {
    let pairs: Vec<Pair> = try_collect(pair)?;
    pairs
}

/// Parse list of paired assignments from `input`.
pub(crate) fn parse_pairs(input: &str) -> Vec<Pair> {
    parse!(pairs: Text::from(input)).expect("Failed to parse list of pair assignments from input")
}
