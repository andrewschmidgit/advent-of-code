use std::collections::HashMap;

pub fn part2(s: &str, limit: usize) -> usize {
    let stones = parse_stones(s);
    count_until(&stones, limit)
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoneChildren {
    Single(Stone),
    Double(Stone, Stone),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Stone {
    value: u64,
    depth: usize,
}

impl Stone {
    pub fn new(value: u64, depth: usize) -> Self {
        Self { value, depth }
    }

    pub fn count(&self, limit: usize, cache: &mut HashMap<Stone, usize>) -> usize {
        if let Some(count) = cache.get(self) {
            return *count;
        }

        let count = if self.depth == limit {
            1
        } else {
            match self.get_children() {
                StoneChildren::Single(stone) => stone.count(limit, cache),
                StoneChildren::Double(stone, stone1) => {
                    stone.count(limit, cache) + stone1.count(limit, cache)
                }
            }
        };

        cache.insert(*self, count);
        count
    }

    fn get_children(&self) -> StoneChildren {
        // rule 1
        if self.value == 0 {
            return StoneChildren::Single(Stone::new(1, self.depth + 1));
        }

        // rule 2
        if let Some((s1, s2)) = self.split() {
            return StoneChildren::Double(s1, s2);
        }

        // rule 3
        StoneChildren::Single(Stone::new(self.value * 2024, self.depth + 1))
    }

    fn split(&self) -> Option<(Stone, Stone)> {
        let mut digits = Vec::new();

        let mut value = self.value;

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

        Some((
            Stone::new(d1, self.depth + 1),
            Stone::new(d2, self.depth + 1),
        ))
    }
}

pub fn count_until(stones: &Vec<Stone>, limit: usize) -> usize {
    let mut cache = HashMap::new();

    stones.iter().map(|s| s.count(limit, &mut cache)).sum()
}

pub fn parse_stones(s: &str) -> Vec<Stone> {
    s.split(" ")
        .filter_map(|s| u64::from_str_radix(s, 10).ok())
        .map(|v| Stone::new(v, 0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_gets_children() {
        let s = Stone::new(0, 1);
        assert_eq!(s.get_children(), StoneChildren::Single(Stone::new(1, 2)));

        let s = Stone::new(20, 1);
        assert_eq!(
            s.get_children(),
            StoneChildren::Double(Stone::new(2, 2), Stone::new(0, 2))
        );

        let s = Stone::new(1, 13);
        assert_eq!(
            s.get_children(),
            StoneChildren::Single(Stone::new(2024, 14))
        );
    }

    #[test]
    fn stone_counts_until() {
        let stones = parse_stones("125 17");
        let count = count_until(&stones, 6);
        assert_eq!(count, 22);
    }
}
