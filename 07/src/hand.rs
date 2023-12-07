use crate::{card::card_value, hand_type::HandType};

#[derive(Debug, Eq)]
pub struct Hand {
    pub bid: u32,
    cards: Vec<u32>,
    t: HandType,
}

impl Hand {
    pub fn new(s: &str, joker: bool) -> Result<Self, String> {
        let (cards, bid) = s.split_once(' ').ok_or("Could not split on whitespace")?;

        let bid = bid.parse().map_err(|_| "Could not parse bid")?;
        let cards = cards.chars().map(|c| card_value(c, joker)).collect();

        let hand_type = HandType::new(&cards, joker)?;

        Ok(Self {
            bid,
            cards,
            t: hand_type,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.t != other.t {
            return self.t.cmp(&other.t);
        }

        let first_unequal = self.cards
            .iter()
            .zip(other.cards.iter())
            .find(|(m, o)| m != o);

        match first_unequal {
            Some((m, o)) => m.cmp(o),
            None => std::cmp::Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;


    #[test]
    fn hand_parses() {
        let cards = vec![ 3, 2, 10, 3, 13, ];

        let t = HandType::new(&cards, false).unwrap();

        let exp = Hand {
            bid: 765,
            cards,
            t,
        };

        let hand = Hand::new("32T3K 765", false).unwrap();

        assert_eq!(exp, hand);
    }

    #[test]
    fn hand_compares() {
        let exps = vec![
            (Ordering::Equal, Hand::new("32T3K 0", false), Hand::new("32T3K 0", false)),
            (Ordering::Greater, Hand::new("KK677 0", false), Hand::new("KTJJT 0", false)),
            (Ordering::Less, Hand::new("32T3K 0", false), Hand::new("T55J5 0", false)),
        ];

        for (ord, one, two) in exps {
            assert_eq!(ord, one.cmp(&two));
        }
    }
}
