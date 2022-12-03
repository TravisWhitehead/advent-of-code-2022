mod parser;

use std::{fmt::Display, fs, ops::Deref};

use parser::parse_elves_calories;

/// A quantity of calories.
type Calories = u32;

/// List of total calories carried by each elf.
struct ElvesTotalCalories(Vec<Calories>);

impl Deref for ElvesTotalCalories {
    type Target = Vec<Calories>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ElvesTotalCalories {
    pub fn greatest_calories_carried(&self) -> Option<&Calories> {
        self.iter().max()
    }
}

impl Display for ElvesTotalCalories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, elf_total_calories) in self.iter().enumerate() {
            writeln!(f, "{elf_total_calories} | {index}")?;
        }
        Ok(())
    }
}

impl From<Vec<Vec<Calories>>> for ElvesTotalCalories {
    fn from(elves_calories: Vec<Vec<Calories>>) -> Self {
        ElvesTotalCalories(
            elves_calories
                .iter()
                .map(|elf_calories| elf_calories.iter().sum())
                .collect(),
        )
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Failed to read input.txt");

    // Parse the input into a list of what each elf recorded (a list of calories of foods they are carrying).
    let elves_calories: Vec<Vec<Calories>> = parse_elves_calories(&input);

    let elves_total_calories = ElvesTotalCalories::from(elves_calories);

    let elf_with_most_calories = elves_total_calories.greatest_calories_carried().unwrap();

    // println!("{}", elves_total_calories);
    println!("{}", elf_with_most_calories);
}
