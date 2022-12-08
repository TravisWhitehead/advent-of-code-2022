use std::fs;

use anyhow::Result;

mod parser;

static INPUT_FILE: &str = "inputs/day5.txt";

type Crate = char;

/// Stack of crates.
struct Stack {
    /// Label refering to a `Stack` in move instructions.
    label: u32,
    crates: Vec<Crate>,
}

struct Stacks {
    stacks: HashMap,
}

impl Stack {}

impl Stacks {
    /// Construct `Stacks` from rows of crates sorted top-down and associated stack labels.
    ///
    /// `crates_grid` is a row of crates where crates may or may not be present.
    /// Assumes `crates_grid` is ordered from top-down meaning the first row (`crates_grid[0]`)
    /// represents crates at the top of the stack, and the final row represents crates at the bottom.
    ///
    /// `stack_labels`
    // TODO: Add example of arguments.
    pub fn from_top_down_crates_rows(
        crates_rows: Vec<Vec<Some(Crate)>>,
        stack_labels: Vec<u32>,
    ) -> Result<Stacks> {
        let ascending_labels = stack_labels.sort();
    }
}

/// Returns the crate that ends up at the top of the stack.
///
/// This solves Day 1 Part 1.
fn top_crate(input_file: &str) {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    todo!()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[ignore]
    #[test]
    fn solve_day_5_part_1() {
        todo!();
    }

    #[ignore]
    #[test]
    fn solve_day_5_part_2() {
        todo!();
    }
}
