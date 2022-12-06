use std::fs;

static INPUT_FILE: &str = "inputs/day5.txt";

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
