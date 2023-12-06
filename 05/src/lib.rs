use map::Map;

mod map;
mod range;

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

    let mut finals = vec![];
    for s in seeds.iter() {
        if print {
            println!();
            println!("Seed: {}", s);
        }
        let mut value = *s;
        for m in maps.iter() {
            if print {
                print!("Map {} | {} -> ", m.title, value);
            }
            value = m.map(&value);
            if print {
                println!("{}", value);
            }
        }

        finals.push(value);
    }

    let lowest = finals.iter().min().expect("Has locations in number");
    println!("solution 1 (lowest location): {}", lowest);
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
