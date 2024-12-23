type Stones = Vec<u64>;

fn main() {
    let mut stones = parse("572556 22 0 528 4679021 1 10725 2790");
    for _ in 0..25 {
        run_rules(&mut stones);
    }

    println!("pt1 stones: {}", stones.len());
}

fn parse(s: &str) -> Stones {
    s.split(" ")
        .filter_map(|s| u64::from_str_radix(s, 10).ok())
        .collect()
}

fn print(stones: &Stones) -> String {
    stones
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn run_rules(stones: &mut Stones) {
    let mut i = 0;
    while i < stones.len() {
        let stone = stones.remove(i);

        // rule 1
        if stone == 0 {
            stones.insert(i, 1);
            i += 1;
            continue;
        }

        // rule 2
        if let Some((d1, d2)) = split_stone(&stone) {
            stones.insert(i, d2);
            stones.insert(i, d1);
            i += 2;

            continue;
        }

        // rule 3
        stones.insert(i, stone * 2024);
        i += 1;
    }
}

fn split_stone(stone: &u64) -> Option<(u64, u64)> {
    let mut digits = Vec::new();

    let mut value = *stone;

    while value > 0 {
        digits.push(value % 10);
        value /= 10;
    }

    let (d2, d1) = digits.split_at(digits.len() / 2);

    if d1.len() != d2.len() {
        return None;
    }

    let d1 = d1
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + d * 10u64.pow(i as u32));

    let d2 = d2
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, d)| acc + d * 10u64.pow(i as u32));

    Some((d1, d2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses() {
        let input = "253 0 2024 14168";
        let stones = parse(&input);
        assert_eq!(stones, vec![253, 0, 2024, 14168]);
    }

    #[test]
    fn stone_splits() {
        assert_eq!(split_stone(&194), None);
        let (s1, s2) = split_stone(&1943).unwrap();

        assert_eq!(s1, 19);
        assert_eq!(s2, 43);
    }

    #[test]
    fn rules_run() {
        let mut stones = parse("125 17");
        run_rules(&mut stones);
        assert_eq!(print(&stones), "253000 1 7");

        run_rules(&mut stones);
        assert_eq!(print(&stones), "253 0 2024 14168");

        run_rules(&mut stones);
        assert_eq!(print(&stones), "512072 1 20 24 28676032");

        run_rules(&mut stones);
        assert_eq!(print(&stones), "512 72 2024 2 0 2 4 2867 6032");

        run_rules(&mut stones);
        assert_eq!(
            print(&stones),
            "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
        );

        run_rules(&mut stones);
        assert_eq!(
            print(&stones),
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }

    #[test]
    fn rules_run_seq() {
        let mut stones = parse("125 17");
        for _ in 0..6 {
            run_rules(&mut stones);
        }

        assert_eq!(stones.len(), 22);

        let mut stones = parse("125 17");
        for _ in 0..25 {
            run_rules(&mut stones);
        }

        assert_eq!(stones.len(), 55312);
    }
}
