use pear::input::{Pear, Text};
use pear::macros::{parse, parse_error, parser};
use pear::{combinators::*, parsers::*};

use crate::Calories;

/// Block of calories lines of foods recorded by a given elf.
type ElfFoodsCalories = Vec<Calories>;

/// Blocks of calories lines of foods recorded by all elves.
type ElvesFoodsCalories = Vec<ElfFoodsCalories>;

/// Use `Text` as the `Input`; this parses `char` tokens.
type Input<'a> = Pear<Text<'a>>;

/// This type alias simplifies parser returns so they primarily express the output type (and lifetime).
///
/// `pear::input::Result` takes the output type `T` and the type of the `Input`.
///
/// See [`pear::macros::parser`] for details.
type Result<'a, T> = pear::input::Result<T, Input<'a>>;

/// Returns true if the char is a digit char (0, 1, 2, ..., 9)
#[inline]
fn is_num_char(&byte: &char) -> bool {
    matches!(byte, '0'..='9')
}

/// Returns true if the char is a newline char (\n)
#[inline]
fn is_newline_char(&byte: &char) -> bool {
    byte == '\n'
}

/// Parser that simply eats a newline char (\n)
#[parser]
fn newline<'a>(input: &mut Input<'a>) -> Result<'a, ()> {
    eat('\n')?;
}

/// Parses a sequence of numeric chars into [`FoodCalories`].
///
/// This takes numeric chars until a newline is encountered, parses the chars as `FoodCalories`,
/// and returns the parsed `FoodCalories` value.
#[parser]
fn calories<'a>(input: &mut Input<'a>) -> Result<'a, Calories> {
    take_some_while_until(is_num_char, '\n')?
        .parse()
        .or_else(|e| parse_error!("{}", e)?)
}

/// Parses calories on a line.
///
/// This parses `calories()`, eats a `newline()` char, and returns the parsed `FoodCalories` value.
#[parser]
fn calories_line<'a>(input: &mut Input<'a>) -> Result<'a, Calories> {
    (calories()?, newline()?).0
}

/// Parses a block of calories lines recorded by an elf into [`ElfCalories`].
///
/// This parses a sequence of calories lines and eats a newline char if the next char matches.
#[parser]
fn elf_calories<'a>(input: &mut Input<'a>) -> Result<'a, ElfFoodsCalories> {
    let calories_lines: ElfFoodsCalories = try_collect(calories_line)?;
    eat_if(is_newline_char)?;
    calories_lines
}

/// Parses blocks of calories lines recorded by all of the elves into [`ElvesCalories`].
///
/// This parses a sequence of blocks of calories lines.
#[parser]
fn elves_calories<'a>(input: &mut Input<'a>) -> Result<'a, ElvesFoodsCalories> {
    let calories_blocks: ElvesFoodsCalories = try_collect(elf_calories)?;
    calories_blocks
}

/// Parse `input` into `ElvesCalories` indicating the calories of the foods carried by each elf.
pub(crate) fn parse_elves_foods_calories(input: &str) -> ElvesFoodsCalories {
    parse!(elves_calories: Text::from(input)).expect("Failed to parse input")
}
