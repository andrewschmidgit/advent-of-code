use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Card {
    winning: Vec<u32>,
    mine: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let count = self.mine
            .iter()
            .filter(|m| self.winning.contains(m))
            .count();

        if count == 0 {
            return 0;
        }

        2u32.pow(count as u32 - 1)
    }

    pub fn wins(&self) -> usize {
        self.mine
            .iter()
            .filter(|m| self.winning.contains(m))
            .count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(':').expect("Should have :");
        let (winning, mine) = rest.split_once('|').expect("Should be separated by |");

        Ok(Card { 
            winning: parse_number_list(winning),
            mine: parse_number_list(mine)
        })
    }
}

fn parse_number_list(s: &str) -> Vec<u32> {
    s
        .split(' ')
        .filter_map(|w| {
            if w.is_empty() {
                return None;
            }

            w.parse::<u32>().ok()
        })
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn card_parses_string() {
        let s = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let exp = Card {
            winning: vec![41, 48, 83, 86, 17],
            mine: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        let card: Card = s.parse().unwrap();

        assert_eq!(exp, card);
    }
}
