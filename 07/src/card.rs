pub fn card_value(c: char, joker: bool) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if joker { 1 } else { 11 },
        'T' => 10,
        '0'..='9' => c.to_digit(10).unwrap(),
        _ => panic!("Not valid Card character")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_parses_from_char() {
        assert_eq!(1, card_value('J', true));
        assert_eq!(3, card_value('3', false));
        assert_eq!(10, card_value('T', false));
        assert_eq!(11, card_value('J', false));
        assert_eq!(12, card_value('Q', false));
        assert_eq!(13, card_value('K', false));
        assert_eq!(14, card_value('A', false));
    }
}
