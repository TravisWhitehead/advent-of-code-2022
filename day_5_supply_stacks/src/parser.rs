#![allow(clippy::result_large_err)]

use pear::{
    combinators::trailing_series,
    input::{Pear, Text},
    macros::{parse_error, parser, switch},
    parsers::*,
};

/// Use `Text` as the `Input`; this parses `char` tokens.
type Input<'a> = Pear<Text<'a>>;

/// See [`pear::macros::parser`] for details.
type Result<'a, T> = pear::input::Result<T, Input<'a>>;

/// Returns true if char is a valid crate label (A through Z).
///
/// This is found in the middle of a stacked crate e.g. [F]
#[inline]
fn is_crate_label_char(&byte: &char) -> bool {
    matches!(byte, 'A'..='Z')
}

/// Returns true if the char is a digit char (0, 1, 2, ..., 9)
#[inline]
fn is_num_char(&byte: &char) -> bool {
    matches!(byte, '0'..='9')
}

/// Parses a stacked crate (e.g. `[U]`)
#[parser]
fn stacked_crate<'a>(input: &mut Input<'a>) -> Result<'a, char> {
    eat('[')?;
    let label = eat_if(is_crate_label_char)
        .or_else(|_| parse_error!("Expected crate label to be char A through Z"))?;
    eat(']')?;
    // surrounded(is_crate_label_char, '')
    // delimited_some('[', is_crate_label_char, ']').map(|label| label[0b0])?
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
fn stacked_crates_row<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Option<char>>> {
    let row: Vec<Option<char>> = trailing_series(some_stacked_crate, ' ')?;
    row
}

/// Parses the label numbers for each stack of crates.
///
/// For example, this parses the numeric chars from the second line in:
/// ```
/// [Z] [M] [P]
///  1   2   3
/// ```
#[parser]
fn stack_labels<'a>(input: &mut Input<'a>) -> Result<'a, Vec<char>> {
    // let labels: Vec<char> = trailing_series(is_num_char, skip_while(' '))?;
    todo!()
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
fn stacked_crates_drawing<'a>(input: &mut Input<'a>) -> Result<'a, Vec<Vec<Option<char>>>> {
    let stacked_crates_rows: Vec<Vec<Option<char>>> = trailing_series(stacked_crates_row, ' ')?;
    stacked_crates_rows
}
