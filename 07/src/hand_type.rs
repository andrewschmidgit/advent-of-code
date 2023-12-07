use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    High,
    OnePair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl HandType {
    pub fn new(cards: &Vec<u32>, _joker: bool) -> Result<Self, String> {
        if cards.len() != 5 {
            return Err(format!("Could not construct type from hand: {:?}", cards));
        }

        let map: HashMap<_, _> = cards.iter().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let (_, value) = map.iter().max_by_key(|pair| pair.1).ok_or("Expected contents")?;

        // TODO jokers

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_type_news_correctly() {
        let exps = vec![
            // (HandType::Five, vec![14, 14, 14, 14, 14]),
            // (HandType::Four, vec![14, 14, 14, 14, 13]),
            // (HandType::Full, vec![14, 14, 14, 13, 13]),
            // (HandType::Three, vec![14, 14, 14, 13, 12]),
            // (HandType::TwoPair, vec![14, 14, 13, 13, 12]),
            // (HandType::OnePair, vec![14, 14, 13, 12, 11]),
            // (HandType::High, vec![14, 13, 12, 11, 10]),
            (HandType::OnePair, vec![3, 2, 10, 3, 13]),
        ];

        for (exp, cards) in exps.into_iter() {
            assert_eq!(exp, HandType::new(&cards, false).unwrap());
        }

        // Now with jokers
        // let exps = vec![
        //     (HandType::Five, vec![14, 14, 14, 14, 1]),
        //     (HandType::Four, vec![14, 14, 14, 1, 13]),
        //     (HandType::Full, vec![14, 14, 1, 13, 13]),
        //     (HandType::Three, vec![14, 14, 1, 13, 12]),
        //     (HandType::TwoPair, vec![14, 1, 13, 13, 12]),
        //     (HandType::OnePair, vec![14, 1, 13, 12, 11]),
        //     (HandType::High, vec![1, 13, 12, 11, 10]),
        // ];
        //
        // for (exp, cards) in exps.into_iter() {
        //     assert_eq!(exp, HandType::new(&cards, false).unwrap());
        // }
    }
}
