use map::Map;

mod map;

pub fn run(contents: &str, print: bool) {
    let seed_line = contents.lines().next().expect("seed line");
    let seeds = seeds(seed_line);
    if print {
        println!("seeds: {:?}", seeds);
    }

    let map_strs = contents
        .split("\n\n")
        .skip(1);

    let maps: Vec<Map> = map_strs.filter_map(|s| s.parse().ok()).collect();

    let min = seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |s, m| m.map(&s)))
        .min().expect("Should be a min");

    println!("solution 1 (lowest location): {}", min);
}

fn seeds(seed_line: &str) -> Vec<u64> {
    let (_, seeds) = seed_line.split_once(':').expect("Seed: should have a ':'");
    seeds
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::seeds;

    #[test]
    fn seed_parses() {
        let s = "seeds: 1 2 3 999 20";
        let exp = vec![1, 2, 3, 999, 20];

        let seeds = seeds(s);

        assert_eq!(exp, seeds);
    }
}
