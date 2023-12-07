use std::collections::HashMap;
use std::str::FromStr;

use crate::card::Card;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl HandType {
    pub fn new(cards: &Vec<Card>) -> Result<Self, String> {
        if cards.len() != 5 {
            return Err(format!("Could not construct type from hand: {:?}", cards));
        }

        let map: HashMap<_, _> = cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let (_, value) = map.iter().max_by(|one, two| one.1.cmp(two.1)).ok_or("Expected contents")?;

        match value {
            5 => Ok(Self::Five),
            4 => Ok(Self::Four),
            3 => Ok(if map.len() == 2 { Self::Full } else { Self::Three }),
            2 => Ok(if map.len() == 3 { Self::TwoPair } else { Self::OnePair }),
            1 => Ok(Self::High),
            _ => Err("Not valid".into())
        }
    }
}

#[derive(Debug, Eq)]
pub struct Hand {
    pub bid: u32,
    cards: Vec<Card>,
    t: HandType,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or("Could not split on whitespace")?;

        let bid = bid.parse().map_err(|_| "Could not parse bid")?;
        let cards = cards.chars().map(|c| c.into()).collect();

        let hand_type = HandType::new(&cards)?;

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
            // flipping because a greater Hand has less unique cards
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
    fn hand_type_news_correctly() {
        let exps = vec![
            (HandType::Five, vec![Card::A, Card::A, Card::A, Card::A, Card::A]),
            (HandType::Four, vec![Card::A, Card::A, Card::A, Card::A, Card::K]),
            (HandType::Full, vec![Card::A, Card::A, Card::A, Card::K, Card::K]),
            (HandType::Three, vec![Card::A, Card::A, Card::A, Card::K, Card::Q]),
            (HandType::TwoPair, vec![Card::A, Card::A, Card::K, Card::K, Card::Q]),
            (HandType::OnePair, vec![Card::A, Card::A, Card::K, Card::Q, Card::J]),
            (HandType::High, vec![Card::A, Card::K, Card::Q, Card::J, Card::T]),
        ];

        for (exp, cards) in exps.into_iter() {
            assert_eq!(exp, HandType::new(&cards).unwrap());
        }
    }

    #[test]
    fn hand_parses() {
        let cards = vec![
            Card::Number(3),
            Card::Number(2),
            Card::T,
            Card::Number(3),
            Card::K,
        ];

        let t = HandType::new(&cards).unwrap();

        let exp = Hand {
            bid: 765,
            cards,
            t,
        };

        let hand: Hand = "32T3K 765".parse().unwrap();

        assert_eq!(exp, hand);
    }

    fn hand(s: &str) -> Hand {
        s.parse().unwrap()
    }

    #[test]
    fn hand_compares() {
        let exps = vec![
            (Ordering::Equal, hand("32T3K 0"), hand("32T3K 0")),
            (Ordering::Greater, hand("KK677 0"), hand("KTJJT 0")),
            (Ordering::Less, hand("32T3K 0"), hand("T55J5 0")),
        ];

        for (ord, one, two) in exps {
            assert_eq!(ord, one.cmp(&two));
        }
    }
}
