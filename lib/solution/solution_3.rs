use crate::{Solution, util::read_lines, error::ConversionError};

pub struct Question3 { }

impl Solution for Question3 {
    fn solve(&self, path: String, verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut score: u32 = 0;
        let lines = read_lines(path)?;

        let mut total_lines = 0;
        let mut hits = 0;
        
        for line in lines {
            let line = line?;
            let backpack = line.as_bytes().split_at(line.len() / 2);
            total_lines += 1;

            'detect: for c1 in backpack.0 {
                for c2 in backpack.1 {
                    if c1 == c2 {
                        score += byte_to_priority(c1)? as u32;
                        hits += 1;
                        break 'detect;
                    }
                }
            }
        }

        if verbose {
            println!("Total lines: {}, Total Hits: {}", total_lines, hits);
        }

        println!("Priority score is {}", score);
        Ok(())
    }

    fn id(&self) -> u32 {
        3
    }
}

fn byte_to_priority(byte: &u8) -> Result<u8, ConversionError> {
    if byte >= &0x41 && byte <= &0x5a {
        // A - Z
        return Ok(byte - 0x26)
    }
    else if byte >= &0x61 && byte <= &0x7a {
        // a - z
        return Ok(byte - 0x60)
    }
    Err(ConversionError::new("byte (u8)", "priority (u8)"))
}