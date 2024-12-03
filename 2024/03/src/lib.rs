use std::{error::Error, num::ParseIntError};

use regex::Regex;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let instructions = parse(&contents)?;

    let sum: i32 = instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Multiply(one, two) => Some(one * two),
            _ => None,
        })
        .sum();

    println!("sum: {}", sum);

    let mut enabled = true;
    let mut sum = 0;
    for i in instructions {
        match i {
            Instruction::Enable => enabled = true,
            Instruction::Disable => enabled = false,
            Instruction::Multiply(one, two) => {
                if !enabled {
                    continue;
                }
                sum += one * two
            }
        }
    }

    println!("sum with toggles: {}", sum);

    Ok(())
}

fn parse(contents: &str) -> Result<Vec<Instruction>, ParseIntError> {
    let re = Regex::new(r"(?:mul\((?<one>[0-9]{1,3}),(?<two>[0-9]{1,3})\))|(?:(?:do|don't)\(\))")
        .unwrap();

    re.captures_iter(contents)
        .map(|c| {
            let extraction = c.get(0).unwrap().as_str();
            let ins = extraction
                .chars()
                .take_while(|&c| c != '(')
                .collect::<String>();
            println!("extraction: {}, ins: {}", extraction, ins);
            match ins.as_str() {
                "do" => Ok(Instruction::Enable),
                "don't" => Ok(Instruction::Disable),
                "mul" => {
                    let one: i32 = c.name("one").unwrap().as_str().parse()?;

                    let two: i32 = c.name("two").unwrap().as_str().parse()?;

                    Ok(Instruction::Multiply(one, two))
                }
                _ => panic!("how did this happen, {}", ins),
            }
        })
        .collect::<Result<Vec<Instruction>, _>>()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Enable,
    Disable,
    Multiply(i32, i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_succeeds() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expectation = vec![
            Instruction::Multiply(2, 4),
            Instruction::Disable,
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Enable,
            Instruction::Multiply(8, 5),
        ];

        let parsed = parse(&input).unwrap();

        assert_eq!(expectation, parsed);
    }
}
