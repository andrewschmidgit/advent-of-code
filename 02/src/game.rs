use std::str::FromStr;

use crate::{set::Set, color::Color};

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self, set: &Set) -> Option<u32> {
        for my_set in self.sets.iter() {
            if !my_set.is_possible_within(set) {
                return None;
            }
        }

        Some(self.id)
    }

    pub fn get_min_set(&self) -> Set {
        let mut red = Color::Red(0);
        let mut green = Color::Green(0);
        let mut blue = Color::Blue(0);

        for set in self.sets.iter() {
            for c in set.colors.iter() {
                match c {
                    Color::Red(v) => {
                        if let Color::Red(c) = red {
                            if c > *v { continue; }
                        }

                        red = Color::Red(*v);
                    },
                    Color::Green(v) => {
                        if let Color::Green(c) = green {
                            if c > *v { continue; }
                        }

                        green = Color::Green(*v);
                    },
                    Color::Blue(v) => {
                        if let Color::Blue(c) = blue {
                            if c > *v { continue; }
                        }

                        blue = Color::Blue(*v);
                    },
                }
            }
        }

        Set { colors: vec![red, green, blue] }
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rest) = s.split_once(": ").ok_or("Could not split on ':'")?;
        let (_, id) = game.split_once(' ').ok_or("Could not get game id")?;

        let sets = rest
            .split("; ")
            .map(|s| s.parse::<Set>())
            .collect::<Result<Vec<Set>, Self::Err>>()?;

        Ok(Game { id: id.parse().map_err(|_| "Could not parse id")?, sets })
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::set::Set;

    use super::*;

    #[test]
    fn game_parses() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let sets = vec![
            Set::new(vec![Color::Blue(3), Color::Red(4)]),
            Set::new(vec![Color::Red(1), Color::Green(2), Color::Blue(6)]),
            Set::new(vec![Color::Green(2)]),
        ];

        let game: Game = line.parse().unwrap();

        assert_eq!(game.id, 1);
        for set in sets {
            assert!(game.sets.contains(&set));
        }
    }

    #[test]
    fn game_1_is_possible() {
        let game: Game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse().unwrap();
        let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert_eq!(game.is_possible(&set), Some(1), "game 1 should've been possible");
    }

    #[test]
    fn game_2_is_possible() {
        let game: Game = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".parse().unwrap();
        let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert_eq!(game.is_possible(&set), Some(2), "game 2 should've been possible");
    }

    #[test]
    fn game_3_is_not_possible() {
        let game: Game = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".parse().unwrap();
        let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert_eq!(game.is_possible(&set), None, "game 3 shouldnt've been possible");
    }

    #[test]
    fn game_4_is_not_possible() {
        let game: Game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".parse().unwrap();
        let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert_eq!(game.is_possible(&set), None, "game 4 shouldnt've been possible");
    }

    #[test]
    fn game_5_is_possible() {
        let game: Game = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".parse().unwrap();
        let set = Set::new(vec![Color::Red(12), Color::Green(13), Color::Blue(14)]);

        assert_eq!(game.is_possible(&set), Some(5), "game 5 should've been possible");
    }

    #[test]
    fn game_gets_minimum_set() {
        let exps = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Set::new(vec![Color::Red(4), Color::Green(2), Color::Blue(6)])),
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Set::new(vec![Color::Red(1), Color::Green(3), Color::Blue(4)])),
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Set::new(vec![Color::Red(20), Color::Green(13), Color::Blue(6)])),
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Set::new(vec![Color::Red(14), Color::Green(3), Color::Blue(15)])),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Set::new(vec![Color::Red(6), Color::Green(3), Color::Blue(2)])),
        ];

        for (input, exp) in exps {
            let game: Game = input.parse().unwrap();
            let min_set = game.get_min_set();

            assert_eq!(exp, min_set, "{input} --- Expected: {:?}, got: {:?}", exp, min_set);
        }
    }
}
