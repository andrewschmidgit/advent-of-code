use std::error::Error;

use itertools::Itertools;
use report::Report;
mod report;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let reports = parse(contents)?;

    let safe_reports = reports.iter().filter(|r| check_safety(&r.levels)).count();

    println!("safe reports: {}/{}", safe_reports, reports.len());

    let tolerant_safe_reports = reports
        .iter()
        .filter(|r| check_safety_tolerant(&r.levels))
        .count();

    println!(
        "safe reports with fault tolerance: {}/{}",
        tolerant_safe_reports,
        reports.len()
    );

    Ok(())
}

fn parse(contents: &str) -> Result<Vec<Report>, Box<dyn Error>> {
    Ok(contents
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Report>, _>>()?)
}

fn check_safety(levels: &Vec<i32>) -> bool {
    // true is increasing, false is decreasing
    let mut direction: Option<bool> = None;

    for pair in levels.windows(2) {
        let a = pair[0];
        let b = pair[1];

        let dir = b > a;
        let diff = (b - a).abs();

        if let Some(direction) = direction {
            if dir != direction {
                return false; // unsafe, direction has changed
            }
        } else {
            direction = Some(dir);
        }

        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

fn check_safety_tolerant(levels: &Vec<i32>) -> bool {
    levels
        .iter()
        .copied()
        .combinations(levels.len() - 1)
        .any(|c| check_safety(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_safety() {
        let cases = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", false),
            ("8 6 4 4 1", false),
            ("1 3 6 7 9", true),
        ];

        for (input, is_safe) in cases {
            println!("input: {}, expected safety: {}", input, is_safe);
            let report: Report = input.parse().unwrap();
            assert_eq!(is_safe, check_safety(&report.levels));
        }
    }

    #[test]
    fn report_tolerant_safety() {
        let cases = vec![
            ("7 6 4 2 1", true),
            ("1 2 7 8 9", false),
            ("9 7 6 2 1", false),
            ("1 3 2 4 5", true),
            ("8 6 4 4 1", true),
            ("1 3 6 7 9", true),
        ];

        for (input, is_safe) in cases {
            println!("input: {}, expected safety: {}", input, is_safe);
            let report: Report = input.parse().unwrap();
            assert_eq!(is_safe, check_safety_tolerant(&report.levels));
        }
    }
}
