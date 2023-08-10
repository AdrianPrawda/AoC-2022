use crate::{Solution, error::ConversionError, util::read_lines};

pub struct Question2 { }

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper,
    Scissor,
}

fn versus(left: Move, right: Move) -> u32 {
    match (left, right) {
        (Move::Rock, Move::Paper) => right as u32 + 6,
        (Move::Rock, Move::Scissor) => right as u32,
        (Move::Paper, Move::Rock) => right as u32,
        (Move::Paper, Move::Scissor) => right as u32 + 6,
        (Move::Scissor, Move::Rock) => right as u32 + 6,
        (Move::Scissor, Move::Paper) => right as u32,
        _ => right as u32 + 3,
    }
}

fn to_moves(i: (&str, &str)) -> Result<(Move, Move), ConversionError> {
    let l: Move = i.0.trim().to_string().try_into()?;
    let r: Move = i.1.trim().to_string().try_into()?;
    Ok((l, r))
}

impl TryFrom<String> for Move {
    type Error = ConversionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
             _  => Err(ConversionError::new(format!("String {}", value).as_str(), "Move"))
        }
    }
}

impl Solution for Question2 {
    fn solve(&self, path: String, _verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut points: u32 = 0;

        let lines = read_lines(path)?;
        for line in lines {
            let l = line?;
            let moves = to_moves(l.split_at(1))?;
            points += versus(moves.0, moves.1);
        }

        println!("Final Score: {}", points);
        Ok(())
    }

    fn id(&self) -> u32 {
        2
    }
}