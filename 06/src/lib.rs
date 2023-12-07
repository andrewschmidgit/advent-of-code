use std::mem::Discriminant;


#[derive(Debug, PartialEq, Eq)]
pub struct Race {
    duration: usize,
    distance: usize,
}

impl Race {
    pub fn new(duration: usize, distance: usize) -> Self {
        Self { duration, distance }
    }

    pub fn wins(&self) -> usize {
        let first_win = (0..=self.duration)
            .find(|t| {
                let traveled = (self.duration - t) * t;
                traveled > self.distance
            });

        match first_win {
            Some(i) => {
                let end_index = self.duration - i;
                end_index - i + 1
            },
            None => 0,
        }
    }
}

pub fn parse(s: &str) -> Vec<Race> {
    let (time, distance) = s.split_once('\n').expect("to be a newline");
    let time = parse_line(time);
    let distance = parse_line(distance);

    time
        .into_iter()
        .zip(distance)
        .map(|(t, d)| Race::new(t, d))
        .collect()
}

fn parse_line(s: &str) -> Vec<usize> {
    let (_, num_s) = s.split_once(':').expect("should be ':'");
    num_s
        .split_whitespace()
        .map(|n| n.parse().expect("should parse into number"))
        .collect()
}

pub fn parse_as_single(s: &str) -> Race {
    let (time, distance) = s.split_once('\n').expect("to be a newline");
    let (_, time) = time.split_once(':').expect("should be a :");
    let (_, distance) = distance.split_once(':').expect("should be a :");

    let time: usize = time.split_whitespace().collect::<String>().parse().expect("Should parse to one number");
    let distance: usize = distance.split_whitespace().collect::<String>().parse().expect("Should parse to one number");

    Race { duration: time, distance }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_parses() {
        let exp = vec![1, 2, 3];

        let result = parse_line("line: 1  2    3");

        assert_eq!(exp, result);
    }

    #[test]
    fn parse_parses() {
        let exp = vec![
            Race::new(7, 9),
            Race::new(15, 40),
            Race::new(30, 200),
        ];

        let result = parse(
r"Time:      7  15   30
Distance:  9  40  200");

        assert_eq!(exp, result);
    }

    #[test]
    fn race_counts_wins() {
        let exps = vec![
            (4, Race::new(7, 9).wins()),
            (8, Race::new(15, 40).wins()),
            (9, Race::new(30, 200).wins()),
        ];

        for (exp, wins) in exps {
            assert_eq!(exp, wins);
        }
    }

    #[test]
    fn parse_single_race_parses() {
        let s = 
r"Time:      7  15   30
Distance:  9  40  200";

        let exp = Race::new(71530, 940200);

        let race = parse_as_single(s);

        assert_eq!(exp, race);
    }
}
