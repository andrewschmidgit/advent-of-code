use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    levels: Vec<i32>,
}

impl Report {
    pub fn is_safe(self: &Self) -> bool {
        // true is increasing, false is decreasing
        let mut direction: Option<bool> = None;

        for pair in self.levels.windows(2) {
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
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|e| e.parse())
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(Report { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_parses() {
        let input = "7 6 4 2 1";
        let expected = Report {
            levels: vec![7, 6, 4, 2, 1],
        };

        let report: Report = input.parse().unwrap();

        assert_eq!(report, expected);
    }

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
            assert_eq!(is_safe, report.is_safe());
        }
    }
}
