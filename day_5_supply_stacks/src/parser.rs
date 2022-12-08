#![allow(clippy::result_large_err)]

use pear::{
    combinators::*,
    input::{Pear, Text},
    macros::{parse_error, parser, switch},
    parsers::*,
};

/// Use `Text` as the `Input`; this parses `char` tokens.
type Input<'a> = Pear<Text<'a>>;

/// See [`pear::macros::parser`] for details.
type Result<'a, T> = pear::input::Result<T, Input<'a>>;

struct Move {
    quantity: u32,
    from_stack: u32,
    to_stack: u32,
}

struct Plan {
    stack_labels: Vec<u32>,
    moves: Vec<Move>,
}

/// Returns true if char is a valid crate label (A through Z).
///
/// This is found in the middle of a stacked crate e.g. [F]
#[inline]
fn is_crate_label(&c: &char) -> bool {
    matches!(c, 'A'..='Z')
}

#[inline(always)]
fn is_num(&c: &char) -> bool {
    c.is_ascii_digit()
}

/// Returns true if the char is a digit char (0, 1, 2, ..., 9)
#[inline(always)]
fn is_space(&c: &char) -> bool {
    c == ' '
}

/// Returns true if the char is a space or a newline.
#[inline(always)]
fn is_whitespace(&c: &char) -> bool {
    c == ' ' || c == '\n'
}

#[parser]
fn number<'a>(input: &mut Input<'a>) -> Result<'a, u32> {
    take_some_while(is_num)?
        .parse()
        .or_else(|e| parse_error!("{}", e)?);
}

/// Parses a stacked crate (e.g. `[U]`)
#[parser]
fn stacked_crate<'a>(input: &mut Input<'a>) -> Result<'a, char> {
    eat('[')?;
    let label = eat_if(is_crate_label)
        .or_else(|_| parse_error!("Expected crate label to be char A through Z"))?;
    eat(']')?;
    label
}

/// Parses either a `stacked_crate` (e.g. `[R]` ) or three spaces (e.g. `   `).
#[parser]
fn some_stacked_crate<'a>(input: &mut Input<'a>) -> Result<'a, Option<char>> {
    switch! {
        eat_slice("   ") => None,
        sc@stacked_crate() => Some(sc),
        _ => parse_error!("Expected crate label (e.g. '[A]' ) or three spaces ('   '), found neither")?,
    }
}

/// Parses a row of `some_stacked_crate` separated by spaces ending with a newline.
///
/// Example:
/// `[Z] [M]     [R]`
#[parser]
fn stacked_crates_line<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Option<char>>> {
    let row: Vec<Option<char>> = trailing_series(some_stacked_crate, ' ')?;
    row
}

/// Parses a stack label (a number indicating the column of the stack) that might be surrounded by spaces.
#[parser]
fn stack_label<'a>(input: &mut Input<'a>) -> Result<'a, u32> {
    skip_while(is_space)?;
    let label = number()?;
    skip_while(is_space)?;
    label
}

/// Parses the label numbers for each stack of crates.
///
/// For example, this parses the numeric chars from the second line in:
/// ```
/// [Z] [M] [P]
///  1   2   3
/// ```
#[parser]
fn stack_labels_line<'a>(input: &mut Input<'a>) -> Result<'a, Vec<u32>> {
    let labels: Vec<u32> = try_collect(stack_label)?;
    labels
}

/// Parses the beginning segment of the input containing a drawing of the arrangement of stacked crates.
///
/// Example:
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
#[parser]
fn stacked_crates_drawing<'a>(
    input: &mut Input<'a>,
) -> Result<'a, (Vec<Vec<Option<char>>>, Vec<u32>)> {
    let stacked_crates_rows = trailing_series(stacked_crates_line, ' ')?;
    let stack_labels = stack_labels_line()?;
    skip_while(is_whitespace)?;
    (stacked_crates_rows, stack_labels)
}

#[parser]
fn move_line<'a>(input: &mut Input<'a>) -> Result<'a, Move> {
    eat_slice("move ")?;
    let quantity = number()?;
    eat_slice(" from ")?;
    let from_stack = number()?;
    eat_slice(" to ")?;
    let to_stack = number()?;
    let move_crates = Move {
        quantity,
        from_stack,
        to_stack,
    };
    eat('\n')?;
    move_crates
}

/// Parse the full plan input.
#[parser]
fn plan<'a>(input: &mut Input<'a>) -> Result<'a, Move> {}
