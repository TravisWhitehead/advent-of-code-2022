mod parser;

use std::{fmt::Display, fs, ops::Deref};

use parser::parse_elves_foods_calories;

static INPUT_FILE: &str = "../inputs/day1.txt";

/// A quantity of calories.
type Calories = u32;

/// List of total calories carried by each elf.
struct ElvesCalories(Vec<Calories>);

impl Deref for ElvesCalories {
    type Target = Vec<Calories>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ElvesCalories {
    pub fn greatest_calories_carried(&self) -> Option<&Calories> {
        self.iter().max()
    }
}

impl Display for ElvesCalories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, elf_calories) in self.iter().enumerate() {
            writeln!(f, "{elf_calories} | {index}")?;
        }
        Ok(())
    }
}

impl From<Vec<Vec<Calories>>> for ElvesCalories {
    fn from(elves_foods_calories: Vec<Vec<Calories>>) -> Self {
        ElvesCalories(
            elves_foods_calories
                .iter()
                .map(|elf_foods_calories| elf_foods_calories.iter().sum())
                .collect(),
        )
    }
}

/// Returns the greatest calories carried by an elf calculated from the data in `input_file`.
///
/// This produces the solution to Day 1 Part 1.
fn greatest_calories_carried(input_file: &str) -> Calories {
    let input = fs::read_to_string(input_file).expect("Failed to read input.txt");

    // Parse the input into a list of what each elf recorded (a list of calories of foods they are carrying).
    let elves_foods_calories: Vec<Vec<Calories>> = parse_elves_foods_calories(&input);

    let elves_calories = ElvesCalories::from(elves_foods_calories);

    *elves_calories.greatest_calories_carried().unwrap()
}

fn main() {
    println!("{}", greatest_calories_carried(INPUT_FILE));
}

#[cfg(test)]
mod test {
    use crate::{greatest_calories_carried, INPUT_FILE};

    #[test]
    fn solve_day_1() {
        assert_eq!(greatest_calories_carried(INPUT_FILE), 67658);
    }
}
