use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Char {
    Match(char),
    Wildcard,
}

fn char_to_char(c: char) -> Char {
    if c == '.' {
        Char::Wildcard
    } else {
        Char::Match(c)
    }
}

#[derive(Debug)]
pub struct Pattern {
    array: Vec<Char>,
    pub width: usize,
}

impl PartialEq<[char]> for Pattern {
    fn eq(&self, other: &[char]) -> bool {
        let same_length = self.array.len() == other.len();
        let chars_eq =
            self.array.iter().zip(other).all(
                |(p, c)| {
                    if let Char::Match(p) = p {
                        p == c
                    } else {
                        true
                    }
                },
            );

        same_length && chars_eq
    }
}

impl Pattern {
    pub fn len(self: &Self) -> usize {
        self.array.len()
    }

    pub fn get_start(self: &Self) -> Option<(usize, char)> {
        self.array.iter().enumerate().find_map(|(i, c)| {
            if let Char::Match(c) = c {
                Some((i, *c))
            } else {
                None
            }
        })
    }
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or("empty string")?.len();

        let array: Vec<_> = s.lines().flat_map(|l| l.chars()).collect();

        if array.len() != width * height {
            return Err("string was not a grid".to_string());
        }

        let array = array.into_iter().map(char_to_char).collect();

        Ok(Self { width, array })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_from_str() {
        let expected_array = vec![
            Char::Match('M'),
            Char::Wildcard,
            Char::Match('S'),
            Char::Wildcard,
            Char::Match('A'),
            Char::Wildcard,
            Char::Match('M'),
            Char::Wildcard,
            Char::Match('S'),
        ];

        let p: Pattern = "M.S
.A.
M.S"
        .parse()
        .unwrap();

        assert_eq!(p.width, 3);

        for (i, c) in p.array.iter().enumerate() {
            assert_eq!(c, &expected_array[i]);
        }
    }

    #[test]
    fn pattern_gets_start_character() {
        let p: Pattern = "...
.A.
..."
        .parse()
        .unwrap();

        let start = p.get_start().unwrap();

        assert_eq!(start, (4, 'A'));
    }

    #[test]
    fn pattern_equality_with_char_array() {
        let p: Pattern = "...
.A.
..."
        .parse()
        .unwrap();

        let chars = ['1', '2', '3', '4', 'A', '6', '7', '8', '9'];

        assert!(p == *chars.as_slice());
    }
}
