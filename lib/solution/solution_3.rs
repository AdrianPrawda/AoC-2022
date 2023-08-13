use crate::{Solution, util::read_lines, error::{ConversionError, SolutionError}};

pub struct Question3 { }

impl Solution for Question3 {
    fn solve(&self, path: String, verbose: bool) -> Result<(), crate::error::InternalSystemError> {
        let mut score: u32 = 0;
        let mut score2: u32 = 0;
        let lines = read_lines(path)?;

        let mut total_lines = 0;
        let mut hits = 0;

        let mut buffer: [Vec<u8>; 3] = [vec![], vec![], vec![]];
        let mut count = 0;
        
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

            let mut line_sorted = line.as_bytes().to_vec();
            line_sorted.sort();
            buffer[count] = line_sorted;
            count += 1;

            if count == 3 {
                match find_common_char(&buffer) {
                    Some(c) => score2 += byte_to_priority(&c)? as u32,
                    None => return Err(SolutionError::from_str("Could not find common char in lines").into()),
                }
                count = 0;
            }
        }

        if verbose {
            println!("Total lines: {}, Total Hits: {}", total_lines, hits);
        }

        println!("Priority score is {}", score);
        println!("Authenticity score is {}", score2);
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

fn find_common_char(lines: &[Vec<u8>; 3]) -> Option<u8> {
    for c1 in lines[0].iter() {
        match lines[1].binary_search(c1) {
            Ok(i1) => {
                match lines[2].binary_search(&lines[1][i1]) {
                    Ok(i2) => return Some(lines[2][i2]),
                    Err(_) => continue,
                }
            },
            Err(_) => continue,
        }
    }
    None
}