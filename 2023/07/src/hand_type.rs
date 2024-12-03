use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl TryFrom<usize> for HandType {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let types = vec![
            HandType::High,
            HandType::OnePair,
            HandType::TwoPair,
            HandType::Three,
            HandType::Full,
            HandType::Four,
            HandType::Five,
        ];

        Ok(types.get(value).ok_or("Could not find HandType from given value")?.clone())
    }
}

impl HandType {
    pub fn new(cards: &Vec<u32>, joker: bool) -> Result<Self, String> {
        if cards.len() != 5 {
            return Err(format!("Could not construct type from hand: {:?}", cards));
        }

        let map: HashMap<_, _> = cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut unique_cards: Vec<_> = map.into_iter().collect();
        unique_cards.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        let most_dups = unique_cards[0];
        let most_dups_2 = unique_cards.get(1).map(|uc| uc.1);

        let joker_count = cards.iter().filter(|c| **c == 1).count() as u32;
        let joker_is_most = *most_dups.0 == 1;

        let most_dups = if joker && !joker_is_most {
            most_dups.1 + joker_count
        } else if joker && joker_is_most && most_dups_2.is_some() {
            most_dups.1 + most_dups_2.unwrap()
        } else {
            most_dups.1
        };

        // This takes care of real Five's and joker induced Five's
        if most_dups >= 5 {
            return Ok(Self::Five);
        }

        let most_dups_2 = most_dups_2.unwrap();

        // This takes care of real Four's and joker induced Four's
        if most_dups == 4 {
            return Ok(Self::Four);
        }

        if most_dups == 3 {
            return Ok(
                if most_dups_2 == 2 {
                    Self::Full
                } else {
                    Self::Three
                }
            );
        }

        if most_dups == 2 {
            return Ok(
                if most_dups_2 == 2 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            );
        }

        Ok(Self::High)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_type_news_correctly() {
        let exps = vec![
            (HandType::Five, vec![14, 14, 14, 14, 14]),
            (HandType::Four, vec![14, 14, 14, 14, 13]),
            (HandType::Full, vec![14, 14, 14, 13, 13]),
            (HandType::Three, vec![14, 14, 14, 13, 12]),
            (HandType::TwoPair, vec![14, 14, 13, 13, 12]),
            (HandType::OnePair, vec![14, 14, 13, 12, 11]),
            (HandType::High, vec![14, 13, 12, 11, 10]),
        ];

        for (exp, cards) in exps.into_iter() {
            assert_eq!(exp, HandType::new(&cards, false).unwrap());
        }

        // Now with jokers
        let exps = vec![
            // 1 joker
            (HandType::Five, vec![14, 14, 14, 14, 1]),
            (HandType::Four, vec![14, 14, 14, 1, 13]),
            (HandType::Full, vec![14, 14, 1, 13, 13]),
            (HandType::Three, vec![14, 14, 1, 13, 12]),
            (HandType::OnePair, vec![14, 1, 13, 12, 11]),
            (HandType::High, vec![14, 13, 12, 11, 10]),

            // 2 jokers
            (HandType::Five, vec![2, 2, 2, 1, 1]),
            (HandType::Four, vec![3, 2, 2, 1, 1]),
            (HandType::Three, vec![4, 3, 2, 1, 1]),

            // 3 jokers
            (HandType::Five, vec![2, 2, 1, 1, 1]),
            (HandType::Four, vec![3, 2, 1, 1, 1]),

            // 4 jokers
            (HandType::Five, vec![2, 1, 1, 1, 1]),

            // 5 jokers
            (HandType::Five, vec![1, 1, 1, 1, 1]),
        ];

        for (exp, cards) in exps.into_iter() {
            assert_eq!(exp, HandType::new(&cards, true).unwrap());
        }
    }
}
