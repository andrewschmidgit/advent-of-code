#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Number(u32),
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '0'..='9' => Card::Number(value.to_digit(10).unwrap()),
            _ => panic!("Not valid Card character")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_orders() {
        // Sanity check
        assert!(Card::A > Card::K);
        assert!(Card::T > Card::Number(4));
        assert!(Card::Number(8) > Card::Number(4));
    }

    #[test]
    fn card_parses_from_char() {
        assert_eq!(Card::Number(3), '3'.into());
        assert_eq!(Card::T, 'T'.into());
        assert_eq!(Card::J, 'J'.into());
        assert_eq!(Card::Q, 'Q'.into());
        assert_eq!(Card::K, 'K'.into());
        assert_eq!(Card::A, 'A'.into());
    }
}
