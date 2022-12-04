mod parser;

use std::{fmt::Display, fs, ops::Deref};

use parser::parse_elves_foods_calories;

// TODO: Be flexible about whether this path is for crate or workspace dir.
static INPUT_FILE: &str = "../inputs/day1.txt";

/// A quantity of calories.
type Calories = u32;

/// List of total calories carried by each elf sorted by descending snack-readiness.
#[derive(Debug)]
struct ElvesCalories(Vec<Calories>);

impl Deref for ElvesCalories {
    type Target = Vec<Calories>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ElvesCalories {
    /// Construct `ElvesCalories` from a list of calories by sorting by descending snack-readiness.
    pub fn new(mut elves_calories: Vec<Calories>) -> ElvesCalories {
        // Sort by descending calories
        elves_calories.sort();
        elves_calories.reverse();

        ElvesCalories(elves_calories)
    }

    /// Returns the calories carried by each of some number of `elves` who are carrying the most calories
    /// (AKA the most snack-ready).
    pub fn most_snack_ready(&self, elves: usize) -> &[Calories] {
        &self[..elves]
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
        // Sum up the calories of the foods each elf is carrying to get the total calories carried by each elf.
        let elves_calories = elves_foods_calories
            .iter()
            .map(|elf_foods_calories| elf_foods_calories.iter().sum())
            .collect();

        ElvesCalories::new(elves_calories)
    }
}

/// Returns [`ElvesCalories`] parsed from input data in `input_file`.
fn elves_calories(input_file: &str) -> ElvesCalories {
    let input = fs::read_to_string(input_file).expect("Failed to read input.txt");

    // Parse the input into a list of what each elf recorded (a list of calories of foods they are carrying).
    let elves_foods_calories: Vec<Vec<Calories>> = parse_elves_foods_calories(&input);

    ElvesCalories::from(elves_foods_calories)
}

/// Returns the greatest calories carried by an elf calculated from the data in `input_file`.
///
/// This produces the solution to Day 1 Part 1.
fn greatest_calories_carried(elves_calories: &ElvesCalories) -> Calories {
    // Return the calories carried by the single most best snack-ready elf
    elves_calories.most_snack_ready(1_usize)[0]
}

/// Returns the total sum of the calories carried by the most snack-ready `number_of_elves`.
///
/// The most snack-ready elves are the elves carrying the most total calories compared to other elves.
///
/// For example, passing `3` for `number_of_elves` returns the total sum of calories carried by the 3 elves
/// who are carrying the most calories individually compared to the other elves.
///
/// This produces the solution to Day 1 Part 2.
fn total_calories_of_most_snack_ready_elves(
    elves_calories: &ElvesCalories,
    elves: usize,
) -> Calories {
    elves_calories.most_snack_ready(elves).iter().sum()
}

fn main() {
    let elves_calories = elves_calories(INPUT_FILE);

    println!(
        "The snack-readiest elf is carrying {} calories.",
        greatest_calories_carried(&elves_calories)
    );
    println!(
        "The 3 most snack-ready elves are carrying a WHOPPING total of {} calories.",
        total_calories_of_most_snack_ready_elves(&elves_calories, 3)
    );
}

#[cfg(test)]
mod test {
    use crate::{
        elves_calories, greatest_calories_carried, total_calories_of_most_snack_ready_elves,
        Calories, ElvesCalories, INPUT_FILE,
    };

    #[test]
    fn day_1_part_1_example() {
        let elves_calories = elves_calories("../inputs/day1-example.txt");
        assert_eq!(greatest_calories_carried(&elves_calories), 24000);
    }

    #[test]
    fn solve_day_1_part_1() {
        let elves_calories = elves_calories(INPUT_FILE);
        assert_eq!(greatest_calories_carried(&elves_calories), 67658);
    }

    #[test]
    fn day_1_part_2_example() {
        let elves_calories = elves_calories("../inputs/day1-example.txt");
        assert_eq!(
            total_calories_of_most_snack_ready_elves(&elves_calories, 3),
            45000
        );
    }

    #[test]
    fn solve_day_1_part_2() {
        let elves_calories = elves_calories(INPUT_FILE);
        assert_eq!(
            total_calories_of_most_snack_ready_elves(&elves_calories, 3),
            200158
        );
    }

    #[test]
    fn elves_calories_is_sorted() {
        let unsorted_elves_calories: Vec<Calories> = vec![2, 1, 7, 3, 0, 5, 9, 4, 6, 8];

        let elves_calories = ElvesCalories::new(unsorted_elves_calories);

        assert_eq!(*elves_calories, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn elves_calories_from_elves_foods_calories() {
        let elves_foods_calories: Vec<Vec<Calories>> =
            vec![vec![100, 200], vec![7, 3], vec![10000]];

        let elves_calories = ElvesCalories::from(elves_foods_calories);

        assert_eq!(*elves_calories, vec![10000, 300, 10]);
    }
}
