use std::{error::Error, iter::zip};

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let (mut list0, mut list1) = parse(contents)?;

    list0.sort();
    list1.sort();

    let distance: i32 = zip(list0, list1)
        .map(|(num0, num1)| (num0 - num1).abs())
        .sum();

    println!("distance: {}", distance);

    Ok(())
}

fn parse(contents: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut list0: Vec<i32> = vec![];
    let mut list1: Vec<i32> = vec![];

    for line in contents.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() != 2 {
            println!("Could not parse line: '{}', skipping", line);
            continue;
        }

        list0.push(numbers[0].parse()?);
        list1.push(numbers[1].parse()?);
    }

    Ok((list0, list1))
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn succeeds() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = parse(input);
        let (list1, list2) = result.expect("to succeed");

        assert_eq!(list1, [3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, [4, 3, 5, 3, 9, 3]);
    }
}
