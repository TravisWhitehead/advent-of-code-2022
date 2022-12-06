use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

static INPUT_FILE: &str = "inputs/day3.txt";

/// Group of three elves with rucksacks that share a common item.
#[derive(Debug)]
struct Group(Rucksack, Rucksack, Rucksack);

#[derive(Debug)]
struct Item(char);

type Priority = u32;

#[derive(Debug)]
struct Rucksack {
    items: String,
}

impl Group {
    /// Returns the common item found in all of the group's rucksacks.
    fn common_item(&self) -> Item {
        for item in self.0.items.chars() {
            if self.1.items.contains(item) && self.2.items.contains(item) {
                return Item(item);
            }
        }
        panic!(
            "No item was found in common in the group's rucksacks: {:?}",
            self
        );
    }
}

impl Item {
    fn priority(&self) -> Priority {
        let offset = if self.0.is_uppercase() { 38 } else { 96 };
        self.0 as Priority - offset
    }
}

impl Rucksack {
    fn left_compartment(&self) -> &str {
        let total_items = &self.items.len();
        let compartment_items = total_items / 2;
        &self.items[0..compartment_items]
    }

    fn right_compartment(&self) -> &str {
        let total_items = self.items.len();
        let compartment_items = total_items / 2;
        &self.items[compartment_items..total_items]
    }

    fn duplicate_item(&self) -> Item {
        for item in self.left_compartment().chars() {
            if self.right_compartment().contains(item) {
                return Item(item);
            }
        }
        panic!("No duplicate item was found in rucksack: {}", self.items);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_groups(input_file: &str) -> Vec<Group> {
    let mut lines = read_lines(input_file).unwrap();
    let mut groups: Vec<Group> = Vec::new();
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        groups.push(Group(
            Rucksack { items: l1.unwrap() },
            Rucksack { items: l2.unwrap() },
            Rucksack { items: l3.unwrap() },
        ));
    }
    groups
}

/// XXX: This does not validate anything (even numbers of items, valid item chars, the input, etc)
fn parse_rucksacks(input_file: &str) -> Vec<Rucksack> {
    read_lines(input_file)
        .unwrap()
        .into_iter()
        .map(|line| Rucksack {
            items: line.unwrap(),
        })
        .collect()
}

/// Returns the sum of the priorities of the common item found in the groups' rucksacks.
///
/// This solves Day 3 Part 2.
fn group_common_items_priority(input_file: &str) -> Priority {
    parse_groups(input_file)
        .iter()
        .map(|g| g.common_item().priority())
        .sum::<Priority>()
}

/// Returns the sum of the priorities of duplicate items that appear in both compartments in Rucksacks.
///
/// This solves Day 3 Part 1.
fn duplicate_items_priority(input_file: &str) -> Priority {
    parse_rucksacks(input_file)
        .iter()
        .map(|r| r.duplicate_item().priority())
        .sum::<Priority>()
}

fn main() {
    println!("The duplicate items that appear in both of the rucksacks' compartments has combined priority: {}\n", duplicate_items_priority(INPUT_FILE));
    println!(
        "The common item found in each group's rucksacks has combined priority: {}",
        group_common_items_priority(INPUT_FILE)
    );
}

#[cfg(test)]
mod test {
    use crate::{duplicate_items_priority, Item, Rucksack, INPUT_FILE};

    #[test]
    fn solve_day_3_part_1() {
        assert_eq!(duplicate_items_priority(INPUT_FILE), 7831);
    }

    #[test]
    fn left_right_rucksack_compartments() {
        let rucksack = Rucksack {
            items: "helloworld".to_string(),
        };

        assert_eq!(rucksack.left_compartment(), "hello");
        assert_eq!(rucksack.right_compartment(), "world");
    }

    #[test]
    fn priorities() {
        assert_eq!(Item('a').priority(), 1);
        assert_eq!(Item('b').priority(), 2);
        assert_eq!(Item('z').priority(), 26);
        assert_eq!(Item('A').priority(), 27);
        assert_eq!(Item('Z').priority(), 52);
    }
}
