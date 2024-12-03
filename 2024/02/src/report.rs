use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Report {
    pub levels: Vec<i32>,
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
}
