#![allow(clippy::result_large_err)]

use pear::{
    combinators::*,
    input::{Cursor, Pear},
    macros::{parse_error, parser},
    parsers::*,
};

use crate::{Move, Round};

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
fn is_newline_byte(&byte: &u8) -> bool {
    byte == b'\n'
}

#[parser]
fn encoded_move<'a>(input: &mut Input<'a>) -> Result<'a, Move> {
    decode_move(&eat_if(is_move_byte)?).or_else(|e| parse_error!("{}", e)?)
}

#[parser]
fn round_line<'a>(input: &mut Input<'a>) -> Result<'a, Round> {
    let opponent = encoded_move()?;
    eat(b' ')?;
    let player = encoded_move()?;
    eat_if(is_newline_byte)?;
    Round { player, opponent }
}

#[parser]
fn strategy_guide<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Round>> {
    let rounds: Vec<Round> = collect(round_line)?;
    rounds
}

pub(crate) fn parse_strategy_guide(input: &str) -> Vec<Round> {
    let mut cursor = Input::new(input.as_bytes());
    strategy_guide(&mut cursor).expect("Failed to parse strategy guide from input")
}
