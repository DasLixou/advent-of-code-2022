use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    /// Gets the weaker move of this (so `self` would win against `get_weaker`)
    pub fn get_weaker(&self) -> Move {
        match *self {
            Move::Scissors => Move::Paper,
            Move::Paper => Move::Rock,
            Move::Rock => Move::Scissors,
        }
    }

    /// Gets the stronger move of this (so `self` would lose against `get_stronger`)
    pub fn get_stronger(&self) -> Move {
        match *self {
            Move::Scissors => Move::Rock,
            Move::Paper => Move::Scissors,
            Move::Rock => Move::Paper,
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(input.to_string()),
        }
    }
}

enum RoundResult {
    Win = 6,
    Tie = 3,
    Lose = 0,
}

fn handle_move(enemy: Move, player: Move) -> RoundResult {
    match player {
        m if enemy == m.get_stronger() => RoundResult::Lose,
        m if enemy == m.get_weaker() => RoundResult::Win,
        _ => RoundResult::Tie,
    }
}

pub fn normal() {
    let input = include_str!("input.txt");
    let result: i32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|m| Move::from_str(m).expect("Unknown Move"))
                .collect();

            handle_move(moves[0], moves[1]) as i32 + moves[1] as i32
        })
        .sum();

    println!("Result: {}", result);
}
