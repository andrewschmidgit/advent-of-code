use std::collections::HashMap;

use card::Card;

mod card;

pub fn run(contents: &str) {
    let cards: Vec<_> = contents
        .lines()
        .filter_map(|l| l.parse::<Card>().ok())
        .collect();

    let sum: u32 = cards
        .iter()
        .map(|c| c.points())
        .sum();

    let mut win_cache: HashMap<usize, usize> = HashMap::new();
    let mut instances = vec![1; cards.len()];

    for (i, c) in cards.iter().enumerate() {
        let wins = match win_cache.get(&i) {
            Some(w) => *w,
            None => c.wins(),
        };
        
        let _ = win_cache.insert(i, wins);

        println!("Card {} wins {}", i + 1, wins);
        for copy_i in (i + 1)..=(i + wins) {
            instances[copy_i] += instances[i];
            println!("updating card {} to have {} copies", i + 1, instances[i]);
        }
    }

    let card_count: usize = instances.iter().sum();

    println!("solution 1 (sum): {}", sum);
    println!("solution 2 (card count): {}", card_count);
}
