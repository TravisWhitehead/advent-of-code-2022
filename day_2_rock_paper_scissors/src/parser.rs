#![allow(clippy::result_large_err)]

use pear::{
    combinators::*,
    input::{Cursor, Pear},
    macros::{parse_error, parser},
    parsers::*,
};

use crate::{Move, Outcome, Round};

fn decode_move(&byte: &u8) -> std::result::Result<Move, &'static str> {
    match byte {
        b'A' => Ok(Move::Rock),
        b'B' => Ok(Move::Paper),
        b'C' => Ok(Move::Scissors),
        b'X' => Ok(Move::Rock),
        b'Y' => Ok(Move::Paper),
        b'Z' => Ok(Move::Scissors),
        _ => Err("{byte} is not a valid encoding of a Rocker Paper Scissors Move"),
    }
}

fn decode_outcome(&byte: &u8) -> std::result::Result<Outcome, &'static str> {
    match byte {
        b'X' => Ok(Outcome::Loss),
        b'Y' => Ok(Outcome::Draw),
        b'Z' => Ok(Outcome::Win),
        _ => Err("{byte} is not a valid encoding of a Rocker Paper Scissors game Outcome"),
    }
}

/// Use cursor over bytes as parser Input.
type Input<'a> = Pear<Cursor<&'a [u8]>>;

/// This type alias simplifies parser returns so they primarily express the output type (and lifetime).
///
/// See [`pear::macros::parser`] for details.
type Result<'a, T> = pear::input::Result<T, Input<'a>>;

#[inline]
fn is_move_byte(&byte: &u8) -> bool {
    matches!(byte, b'A'..=b'C') || matches!(byte, b'X'..=b'Z')
}

#[inline]
fn is_outcome_byte(&byte: &u8) -> bool {
    matches!(byte, b'X'..=b'Z')
}

#[inline]
fn is_newline_byte(&byte: &u8) -> bool {
    byte == b'\n'
}

#[parser]
fn encoded_move<'a>(input: &mut Input<'a>) -> Result<'a, Move> {
    decode_move(&eat_if(is_move_byte)?).or_else(|e| parse_error!("{}", e)?)
}

#[parser]
fn encoded_outcome<'a>(input: &mut Input<'a>) -> Result<'a, Outcome> {
    decode_outcome(&eat_if(is_outcome_byte)?).or_else(|e| parse_error!("{}", e)?)
}

/// Parse the line as the opponent's move and the player's move.
#[parser]
fn round_moves_line<'a>(input: &mut Input<'a>) -> Result<'a, Round> {
    let opponent = encoded_move()?;
    eat(b' ')?;
    let player = encoded_move()?;
    eat_if(is_newline_byte)?;
    Round { player, opponent }
}

/// Parse the line as the opponent's move and the desired game outcome.
#[parser]
fn round_outcome_line<'a>(input: &mut Input<'a>) -> Result<'a, (Move, Outcome)> {
    let opponent_move = encoded_move()?;
    eat(b' ')?;
    let outcome = encoded_outcome()?;
    eat_if(is_newline_byte)?;
    (opponent_move, outcome)
}

#[parser]
fn moves_strategy_guide<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Round>> {
    let rounds: Vec<Round> = collect(round_moves_line)?;
    rounds
}

#[parser]
fn outcomes_strategy_guide<'a>(input: &mut Input<'a>) -> Result<'a, Vec<(Move, Outcome)>> {
    let rounds: Vec<(Move, Outcome)> = collect(round_outcome_line)?;
    rounds
}

pub(crate) fn parse_moves_strategy_guide(input: &str) -> Vec<Round> {
    let mut cursor = Input::new(input.as_bytes());
    moves_strategy_guide(&mut cursor).expect("Failed to parse moves strategy guide from input")
}

pub(crate) fn parse_outcomes_strategy_guide(input: &str) -> Vec<(Move, Outcome)> {
    let mut cursor = Input::new(input.as_bytes());
    outcomes_strategy_guide(&mut cursor)
        .expect("Failed to parse outcomes strategy guide from input")
}
