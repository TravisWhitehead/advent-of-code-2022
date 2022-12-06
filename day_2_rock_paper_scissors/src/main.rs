use std::{cmp::Ordering, fs};

use parser::{parse_moves_strategy_guide, parse_outcomes_strategy_guide};

mod parser;

static INPUT_FILE: &str = "inputs/day2.txt";

type Score = u32;

/// Possible moves with different score values in Rock Paper Scissors.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// The outcome and score of a round of Rock Paper Scissors.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Draw = 3,
    Loss = 0,
    Win = 6,
}

/// The moves played by both players in a round of Rock Paper Scissors.
#[derive(Debug, PartialEq)]
struct Round {
    player: Move,
    opponent: Move,
}

impl Move {
    /// Returns which `Move` this `Move` beats in a game of Rock Paper Scissors.
    fn beats(&self) -> Move {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }

    /// Returns which `Move` this `Move` loses against in a game of Rock Paper Scissors.
    fn loses(&self) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn score(&self) -> Score {
        *self as Score
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Outcome {
    /// Returns the score associated with a game Outcome.
    fn score(&self) -> Score {
        *self as Score
    }

    /// Returns the player's move that would produce this Outcome against `opponent_move`.
    fn player_move(&self, opponent_move: &Move) -> Move {
        match self {
            Outcome::Draw => *opponent_move,
            Outcome::Loss => opponent_move.beats(),
            Outcome::Win => opponent_move.loses(),
        }
    }
}

impl Round {
    /// Returns the outcome of the player's move against the opponent's move.
    fn outcome(&self) -> Outcome {
        if self.player == self.opponent {
            Outcome::Draw
        } else if self.player > self.opponent {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    /// Returns the player's score for the round.
    ///
    /// The round score is the total of the points scored for the player's chosen
    /// move and the score for the outcome of the game.
    fn score(&self) -> Score {
        self.player.score() + self.outcome().score()
    }
}

fn rounds_from_moves_strategy_guide(input_file: &str) -> Vec<Round> {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    parse_moves_strategy_guide(&input)
}

fn rounds_from_outcomes_strategy_guide(input_file: &str) -> Vec<Round> {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    let strategy_guide = parse_outcomes_strategy_guide(&input);
    strategy_guide
        .iter()
        .map(|(opponent_move, outcome)| Round {
            player: outcome.player_move(opponent_move),
            opponent: *opponent_move,
        })
        .collect()
}

/// Returns the final score of multiple rounds of rock paper scissors.
fn total_score(rounds: Vec<Round>) -> Score {
    rounds.iter().map(|round| round.score()).sum()
}

fn main() {
    println!(
        "Interpreting the strategy guide as opponent moves to chosen moves would result in a final score of {}.",
         total_score(rounds_from_moves_strategy_guide(INPUT_FILE))
    );
    println!(
        "Interpreting the strategy guide as opponent moves to desired outcomes would result in a final score of {}.",
        total_score(rounds_from_outcomes_strategy_guide(INPUT_FILE))
    )
}

#[cfg(test)]
mod test {
    static EXAMPLE_INPUT_FILE: &str = "inputs/day2-example.txt";

    use crate::{
        rounds_from_moves_strategy_guide, rounds_from_outcomes_strategy_guide, total_score,
        INPUT_FILE,
    };

    #[test]
    fn solve_day_2_part_1() {
        assert_eq!(
            total_score(rounds_from_moves_strategy_guide(INPUT_FILE)),
            14264
        )
    }

    #[test]
    fn solve_day_2_example_part_1() {
        assert_eq!(
            total_score(rounds_from_moves_strategy_guide(EXAMPLE_INPUT_FILE)),
            15
        )
    }

    #[test]
    fn solve_day_2_part_2() {
        assert_eq!(
            total_score(rounds_from_outcomes_strategy_guide(INPUT_FILE)),
            12382
        )
    }

    #[test]
    fn solve_day_2_example_part_2() {
        assert_eq!(
            total_score(rounds_from_outcomes_strategy_guide(EXAMPLE_INPUT_FILE)),
            12
        )
    }
}
