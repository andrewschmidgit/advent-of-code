use std::{fs, error::Error};

const DIGIT_STR: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("./input.txt")?;

    let digits: Vec<String> = DIGIT_STR.iter().map(|s| String::from(s.to_owned())).collect();
    let rev_digits: Vec<String> = DIGIT_STR.iter()
        .map(|s| {
            s.chars().rev().collect::<String>()
        })
        .collect();

    let mut solution_1 = 0;
    let mut solution_2 = 0;
    for line in contents.lines() {
        let reversed_line: String = String::from(line).chars().rev().collect();

        let start_1 = find_digit_1(line).unwrap();
        let end_1 = find_digit_1(&reversed_line).unwrap();
        solution_1 += 10 * start_1 + end_1;

        let start_2 = find_digit_2(line, &digits).unwrap();
        let end_2 = find_digit_2(&reversed_line, &rev_digits).unwrap();
        solution_2 += 10 * start_2 + end_2;
    }

    println!("solution 1: {}", solution_1);
    println!("solution 2: {}", solution_2);

    Ok(())
}

fn find_digit_1(line: &str) -> Option<usize> {
    let mut index = line.len();
    let mut value = None;

    for d in 1..=9 {
        if let Some(found_i) = line.find(&d.to_string()) {
            if found_i <= index {
                index = found_i;
                value = Some(d);
            }
        }
    }

    value
}

fn find_digit_2(line: &str, digits: &[String]) -> Option<usize> {
    let mut index = line.len();
    let mut value = None;

    for (i, d) in digits.iter().enumerate() {
        let i = i + 1; // Actual number value
        if let Some(found_i) = line.find(d) {
            if found_i <= index {
                index = found_i;
                value = Some(i);
            }
        }

        if let Some(found_i) = line.find(&i.to_string()) {
            if found_i <= index {
                index = found_i;
                value = Some(i);
            }
        }
    }

    value
}
