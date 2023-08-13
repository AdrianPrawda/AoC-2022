use crate::{Solution, util::read_lines};

pub struct Question4 { }

impl Solution for Question4 {
    fn solve(&self, path: String, _verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut score: u32 = 0;
        let mut score2: u32 = 0;
        let lines = read_lines(path)?;

        for line in lines {
            let line = line?.replace("-", ",");

            let n: Result<Vec<u16>, _> = line.split(",").map(|s| {
                s.parse::<u16>()
            }).collect();
            let n = n?;

            if contains_subset(&n[0], &n[1], &n[2], &n[3]) {
                score += 1;
            }
            if contains_overlap(&n[0], &n[1], &n[2], &n[3]) {
                score2 += 1;
            }

        }

        println!("There are {} subset assignements", score);
        println!("There are {} overlaping assignements", score2);
        Ok(())
    }

    fn id(&self) -> u32 {
        4
    }
}

fn contains_subset(a1: &u16, a2: &u16, b1: &u16, b2: &u16) -> bool {
    (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
}

fn contains_overlap(a1: &u16, a2: &u16, b1: &u16, b2: &u16) -> bool {
    b2 >= a1 && b1 <= a2
}