use std::fs;

use parser::parse_moves_strategy_guide;

mod parser;

static INPUT_FILE: &str = "../inputs/day2.txt";

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

    fn score(&self) -> Score {
        *self as Score
    }
}

impl Outcome {
    /// Returns the score associated with a game Outcome.
    fn score(&self) -> Score {
        *self as Score
    }
}

impl Round {
    /// Returns the outcome of the player's move against the opponent's move.
    fn outcome(&self) -> Outcome {
        if self.player == self.opponent {
            return Outcome::Draw;
        }

        if self.player.beats() == self.opponent {
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

fn moves_strategy_guide(input_file: &str) -> Vec<Round> {
    let input = fs::read_to_string(input_file).expect("Failed to read input file");
    parse_moves_strategy_guide(&input)
}

/// Returns the final score of multiple rounds of rock paper scissors.
fn total_score(rounds: Vec<Round>) -> Score {
    rounds.iter().map(|round| round.score()).sum()
}

fn main() {
    let final_score = total_score(moves_strategy_guide(INPUT_FILE));
    println!(
        "Following the strategy guide would result in a final score of {}.",
        final_score
    )
}

#[cfg(test)]
mod test {
    use crate::{moves_strategy_guide, total_score, INPUT_FILE};

    #[test]
    fn solve_day_2() {
        assert_eq!(total_score(moves_strategy_guide(INPUT_FILE)), 14264)
    }

    #[test]
    fn solve_day_2_example() {
        assert_eq!(
            total_score(moves_strategy_guide("../inputs/day2-example.txt")),
            15
        )
    }
}
