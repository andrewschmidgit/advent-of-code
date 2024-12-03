use std::{error::Error, num::ParseIntError};

use regex::Regex;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let parsed = parse(&contents)?;

    let sum: i32 = parsed.iter().map(|(a, b)| a * b).sum();

    println!("sum: {}", sum);

    Ok(())
}

fn parse(contents: &str) -> Result<Vec<(i32, i32)>, ParseIntError> {
    let re = Regex::new(r"mul\((?<one>[0-9]{1,3}),(?<two>[0-9]{1,3})\)").unwrap();
    re.captures_iter(contents)
        .map(|c| {
            let one: i32 = c.name("one").unwrap().as_str().parse()?;

            let two: i32 = c.name("two").unwrap().as_str().parse()?;

            Ok((one, two))
        })
        .collect::<Result<Vec<(i32, i32)>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_succeeds() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expectation = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let parsed = parse(&input).unwrap();

        assert_eq!(expectation, parsed);
    }
}
