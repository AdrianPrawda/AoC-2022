use std::cmp::max;

use crate::{Solution, util::read_lines};

pub struct Question1 { }

impl Question1 {
    pub fn new() -> Self {
        Question1 {  }
    }
}

impl Solution for Question1 {
    fn solve(&self, path: String, _verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut calories = 0;
        let mut max_cal = calories;

        let lines = read_lines(path)?;
        for line in lines {
            let lstr = line?;
            if lstr.is_empty() {
                max_cal = max(calories, max_cal);
                calories = 0;
            } else {
                calories += lstr.parse::<i32>()?;
            }
        }

        println!("Maximum Calories: {}", max_cal);
        Ok(())
    }

    fn id(&self) -> u32 {
        1
    }
}