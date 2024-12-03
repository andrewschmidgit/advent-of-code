use crate::map::{Map, Direction};

pub fn traverse<'a>(map: &'a Map, start: &'a str) -> Result<u64, String> {
    let mut current = start;
    let mut steps = 0;

    for d in map.directions() {
        if current.ends_with('Z') { break; }
        steps += 1;

        let (l, r) = map.nodes.get(current).ok_or("Expected node")?;

        current = match d {
            Direction::Left => l,
            Direction::Right => r,
        }
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverses() {
        let map: Map = 
r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".parse().unwrap();

        let start = "AAA";
        let exp = 2;
        let traversal = traverse(&map, start.into()).unwrap();

        assert_eq!(exp, traversal);
    }
}
