use std::{ops::Range, sync::Mutex};

use linya::{Progress, Bar};
use map::Map;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod map;

pub fn run(contents: &str) {
    let seed_line = contents.lines().next().expect("seed line");
    let seeds = seeds(seed_line);

    let map_strs = contents
        .split("\n\n")
        .skip(1);

    let maps: Vec<Map> = map_strs.filter_map(|s| s.parse().ok()).collect();

    let min = seeds
        .into_iter()
        .map(|s| maps.iter().fold(s, |s, m| m.map(&s)))
        .min().expect("Should be a min");

    println!("solution 1 (lowest location): {}", min);

    let seeds = seed_ranges(seed_line);
    
    let progress = Mutex::new(Progress::new());
    let min = seeds
        .into_par_iter()
        .map(|r| {
            let total = r.end as usize - r.start as usize;
            let bar: Bar = progress.lock().unwrap().bar(total, format!("Processing seeds {} to {}", r.start, r.end));
            r.enumerate().map(|(i, s)| {
                let l = maps.iter().fold(s, |s, m| m.map(&s));
                if i % 1000 == 0 {
                    progress.lock().unwrap().inc_and_draw(&bar, 1000);
                }
                l
            }).min().unwrap()
        })
        .min().expect("should be a min");

    println!("solution 2 (lowest location): {}", min);
}

fn seeds(seed_line: &str) -> Vec<u64> {
    let (_, seeds) = seed_line.split_once(':').expect("Seed: should have a ':'");
    seeds
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn seed_ranges(s: &str) -> Vec<Range<u64>> {
    let (_, seeds) = s.split_once(':').expect("Seed: should have a ':'");
    let seeds: Vec<_> = seeds
        .split_whitespace()
        .map(|s| s.parse().expect("Should parse to number"))
        .collect();
        
    seeds
        .chunks_exact(2)
        .map(|c| c[0]..(c[0] + c[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_parses() {
        let s = "seeds: 1 2 3 999 20";
        let exp = vec![1, 2, 3, 999, 20];

        let seeds = seeds(s);

        assert_eq!(exp, seeds);
    }

    #[test]
    fn seed_ranges_parses() {
        let s = "seeds: 0 10 10 100";
        let exp = vec![
            0..10,
            10..110
        ];

        let seeds = seed_ranges(s);

        assert_eq!(exp, seeds);
    }
}
