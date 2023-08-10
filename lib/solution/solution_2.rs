use crate::{Solution, error::ConversionError, util::read_lines};

pub struct Question2 { }

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper,
    Scissor,
}

#[derive(Debug, Clone, Copy)]
enum GameOutcome {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn versus(left: Move, right: Move) -> u32 {
    match (left, right) {
        (Move::Rock, Move::Paper) => right as u32 + GameOutcome::Win as u32,
        (Move::Rock, Move::Scissor) => right as u32,
        (Move::Paper, Move::Rock) => right as u32,
        (Move::Paper, Move::Scissor) => right as u32 + GameOutcome::Win as u32,
        (Move::Scissor, Move::Rock) => right as u32 + GameOutcome::Win as u32,
        (Move::Scissor, Move::Paper) => right as u32,
        _ => right as u32 + GameOutcome::Draw as u32,
    }
}

fn to_moves(i: (&str, &str)) -> Result<(Move, Move), ConversionError> {
    let l: Move = i.0.to_string().try_into()?;
    let r: Move = i.1.to_string().try_into()?;
    Ok((l, r))
}

fn outcome_to_move(mov: Move, outcome: GameOutcome) -> Move {
    match outcome {
        GameOutcome::Win => {
            match mov {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissor,
                Move::Scissor => Move::Rock,
            }
        },
        GameOutcome::Lose => {
            match mov {
                Move::Rock => Move::Scissor,
                Move::Paper => Move::Rock,
                Move::Scissor => Move::Paper,
            }
        },
        GameOutcome::Draw => mov.clone(),
    }
}

impl TryFrom<String> for Move {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str().trim() {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
             _  => Err(ConversionError::new(format!("String {}", value).as_str(), "Move"))
        }
    }
}

impl TryFrom<String> for GameOutcome {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str().trim() {
            "X" => Ok(GameOutcome::Lose),
            "Y" => Ok(GameOutcome::Draw),
            "Z" => Ok(GameOutcome::Win),
            _  => Err(ConversionError::new(format!("String {}", value).as_str(), "GameOutcome")),
        }
    }
}

impl Solution for Question2 {
    fn solve(&self, path: String, _verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut points_a: u32 = 0;
        let mut points_b: u32 = 0;

        let lines = read_lines(path)?;
        for line in lines {
            let l = line?;
            let values = l.split_at(1);

            let moves = to_moves(values)?;
            points_a += versus(moves.0, moves.1);

            let outcome: GameOutcome = values.1.to_owned().try_into()?;
            points_b += outcome as u32 + outcome_to_move(moves.0, outcome) as u32;
        }

        println!("Final Score (1st Strategy): {}", points_a);
        println!("Final Score (2st Strategy): {}", points_b);
        Ok(())
    }

    fn id(&self) -> u32 {
        2
    }
}