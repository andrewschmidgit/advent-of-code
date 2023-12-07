use hand::Hand;

mod card;
mod hand;
mod hand_type;

pub fn parse(s: &str, joker: bool) -> Vec<Hand> {
    s.lines()
        .filter_map(|l| Hand::new(l, joker).ok())
        .collect()
}
