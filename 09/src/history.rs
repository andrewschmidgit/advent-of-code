use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct History {
    pub values: Vec<i32>,
}

impl History {
    fn sequence(&self) -> Vec<Vec<i32>> {
        let mut sequences = vec![self.values.clone()];

        while let Some(l) = sequences.last() {
            let sequence: Vec<_> = l
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();

            let all_zeros =  sequence.iter().all(|s| *s == 0);

            sequences.push(sequence);

            if all_zeros { break; }
        }

        sequences
    }

    pub fn predict_prev(&self) -> i32 {
        let sequences = self.sequence();

        sequences
            .iter()
            .rev()
            .fold(0, |acc, s| {
                s.first().unwrap() - acc
            })
    }

    pub fn predict_next(&self) -> i32 {
        let sequences = self.sequence();

        sequences
            .iter()
            .rev()
            .fold(0, |acc, s| {
                acc + s.last().unwrap()
            })
    }
}

impl FromStr for History {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Self {
            values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_parses() {
        let s = "0 3 6 9 12 15";
        let exp = History {
            values: vec![0, 3, 6, 9, 12, 15]
        };

        let history: History = s.parse().unwrap();

        assert_eq!(exp, history);
    }

    fn h(s: &str) -> History {
        s.parse().unwrap()
    }

    #[test]
    fn history_sequences() {
        let history: History = h("0 3 6 9 12 15");
        let exp = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];

        let sequences = history.sequence();

        assert_eq!(exp, sequences);
    }

    #[test]
    fn history_predicts_prev() {
        let exps = vec![
            (-3, h("0 3 6 9 12 15")),
            (0, h("1 3 6 10 15 21")),
            (5, h("10 13 16 21 30 45")),
        ];

        for (e, h) in exps.into_iter() {
            let p = h.predict_prev();

            assert_eq!(e, p);
        }
    }
}
