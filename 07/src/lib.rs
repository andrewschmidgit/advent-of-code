use hand::Hand;

mod card;
mod hand;
mod hand_type;

fn parse(s: &str, joker: bool) -> Vec<Hand> {
    s.lines()
        .filter_map(|l| Hand::new(l, joker).ok())
        .collect()
}

pub fn run(s: &str, joker: bool) -> usize {
    let mut hands = parse(s, joker);

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let rank = i + 1;
            rank * h.bid as usize
        })
        .sum()
}
