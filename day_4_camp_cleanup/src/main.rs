use std::{fs, ops::RangeInclusive};

use parser::parse_pairs;

mod parser;

static INPUT_FILE: &str = "inputs/day4.txt";

/// A range of `Section` IDs that an Elf is assigned to.
type Assignment = RangeInclusive<Section>;

/// A `Pair` of two Elves's assignments.
struct Pair(Assignment, Assignment);

/// Section ID that an Elf is assigned to.
type Section = u32;

impl Pair {
    /// Returns whether the pair's assignments are redundant.
    ///
    /// This returns true when one of the assignments's section IDs are all included in the other assignment.
    fn is_redundant(&self) -> bool {
        // If either assignment range contains the start and end of the other assignment, the pair is redundant.
        if self.0.contains(self.1.start()) && self.0.contains(self.1.end()) {
            return true;
        };
        if self.1.contains(self.0.start()) && self.1.contains(self.0.end()) {
            return true;
        };
        false
    }
}

/// Returns the number of redunant pairs found in the `input_file`.
///
/// This solves Day 4 Part 1.
fn count_redundant_pairs(input_file: &str) -> usize {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    parse_pairs(&input)
        .iter()
        .filter(|p| p.is_redundant())
        .count()
}

fn main() {
    println!(
        "The assignment list has {} redundant pairs of assignments.",
        count_redundant_pairs(INPUT_FILE)
    );
}

#[cfg(test)]
mod test {
    use crate::{count_redundant_pairs, INPUT_FILE};

    #[test]
    fn solve_day_4_part_1() {
        assert_eq!(count_redundant_pairs(INPUT_FILE), 602);
    }

    #[ignore]
    #[test]
    fn solve_day_4_part_2() {
        todo!()
    }
}
