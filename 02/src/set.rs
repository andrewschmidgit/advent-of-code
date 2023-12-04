use std::str::FromStr;

use crate::color::Color;

#[derive(Debug, PartialEq)]
pub struct Set {
    colors: Vec<Color>
}

impl Set {
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    pub fn is_possible_within(&self, other: &Self) -> bool {
        let available_colors: Vec<_> = other.colors
            .iter()
            .map(std::mem::discriminant)
            .collect();

        for c in self.colors.iter() {
            if !available_colors.contains(&std::mem::discriminant(c)) {
                println!("My color: {:?} not found within {:?}", c, available_colors);
                return false;
            }

            for oc in other.colors.iter() {
                if let (Color::Red(v), Color::Red(ov)) = (c, oc) {
                    // println!("Comparing my red: {} to avail red: {}", v, ov);
                    if v > ov { return false };
                }
                if let (Color::Green(v), Color::Green(ov)) = (c, oc) {
                    // println!("Comparing my green: {} to avail green: {}", v, ov);
                    if v > ov { return false };
                }
                if let (Color::Blue(v), Color::Blue(ov)) = (c, oc) {
                    // println!("Comparing my blue: {} to avail blue: {}", v, ov);
                    if v > ov { return false };
                }
            }
        }

        true
    }
}

impl FromStr for Set {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors = s
            .split(", ")
            .map(|c| c.parse::<Color>())
            .collect::<Result<Vec<Color>, Self::Err>>()?;

        Ok(Self { colors })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_parses() {
        let expectations = vec![
            ("3 blue, 4 red", Set { colors: vec![Color::Blue(3), Color::Red(4)] }),
            ("1 red, 2 green, 6 blue", Set { colors: vec![Color::Red(1), Color::Green(2), Color::Blue(6)]}),
            ("2 green", Set { colors: vec![Color::Green(2)] }),
        ];

        for (str, ex) in expectations {
            assert_eq!(str.parse::<Set>().unwrap(), ex);
        }
    }

    #[test]
    fn set_impossible_value_single() {
        let me = Set::new(vec![Color::Red(2)]);
        let other = Set::new(vec![Color::Red(1)]);

        assert!(!me.is_possible_within(&other), "2 red should not be possible within 1 red");
    }

    #[test]
    fn set_impossible_variant_single() {
        let me = Set::new(vec![Color::Green(1)]);
        let other = Set::new(vec![Color::Red(2)]);

        assert!(!me.is_possible_within(&other), "1 green should not be possible within 2 red");
    }

    #[test]
    fn set_impossible_game_3() {
        let me = Set::new(vec![Color::Green(8), Color::Blue(6), Color::Red(20)]);
        let other = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert!(!me.is_possible_within(&other), "game 3 should be impossible");
    }

    #[test]
    fn set_possible_value_single() {
        let me = Set::new(vec![Color::Red(1)]);
        let other = Set::new(vec![Color::Red(2)]);

        assert!(me.is_possible_within(&other), "1 red should be possible within 2 red");
    }

    #[test]
    fn set_possible_value_multiple() {
        let me = Set::new(vec![Color::Red(2)]);
        let other = Set::new(vec![Color::Red(2), Color::Green(1)]);

        assert!(me.is_possible_within(&other), "2 red should be possible within 2 red, 1 green");
    }

    #[test]
    fn set_possible_misordered() {
        let me = Set::new(vec![Color::Red(2)]);
        let other = Set::new(vec![Color::Green(1), Color::Red(2), Color::Blue(3)]);

        assert!(me.is_possible_within(&other), "2 red should be possible within 1 green, 2 red, 3 blue");
    }

    #[test]
    fn set_is_possible() {
        let failures = vec![
            (Set::new(vec![Color::Red(2)]), Set::new(vec![Color::Red(1)]), "Value comparison failed"),
            (Set::new(vec![Color::Green(0)]), Set::new(vec![Color::Red(1)]), "Variant comparison failed"),
        ];

        let successes = vec![
            (Set::new(vec![Color::Red(1)]), Set::new(vec![Color::Red(2)]), "Value comparison failed"),
            (Set::new(vec![Color::Green(3)]), Set::new(vec![Color::Red(1), Color::Green(3)]), "Variant list comparison failed"),
        ];

        for (set, other, m) in failures {
            assert!(!set.is_possible_within(&other), "{}", m);
        }

        for (set, other, m) in successes {
            assert!(set.is_possible_within(&other), "{}", m);
        }
    }
}
