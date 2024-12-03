use std::{str::FromStr, ops::Range};

#[derive(Debug, PartialEq)]
pub struct Map {
    pub title: String,
    ranges: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    pub fn map(&self, seed: &u64) -> u64 {
        let range = self.ranges.iter().find(|(src, _)| src.contains(seed));

        if let Some((src, dest)) = range {
            return seed - src.start + dest.start;
        }

        *seed
    }
}

fn parse_mapping(s: &str) -> Result<(Range<u64>, Range<u64>), String> {
    let parts: Vec<u64> = s
        .splitn(3, ' ')
        .filter_map(|s| s.parse().ok())
        .collect();

    let dest = parts.first().ok_or("expected dest")?;
    let src = parts.get(1).ok_or("expected src")?;
    let length = parts.get(2).ok_or("expected length")?;


    Ok((
        *src..(src + length),
        *dest..(dest + length),
    ))
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let title = lines.next().ok_or("Should have a title line")?.to_owned();

        let maps = lines
            .filter_map(|l| parse_mapping(l).ok())
            .collect();
        
        Ok(Map { 
            title,
            ranges: maps,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_parses() {
        let s = 
r"test to test map:
1 2 3
4 5 6
10 11 1";

        let exp = Map {
            title: "test to test map:".into(),
            ranges: vec![
                parse_mapping("1 2 3").unwrap(),
                parse_mapping("4 5 6").unwrap(),
                parse_mapping("10 11 1").unwrap(),
            ]
        };

        let map: Map = s.parse().unwrap();

        assert_eq!(exp, map);
    }
    
    #[test]
    fn map_maps() {
        let map: Map = 
r"test to test map:
1 2 3
4 5 6
10 11 1".parse().unwrap();

        assert_eq!(0, map.map(&0));
        assert_eq!(6, map.map(&7));
    }
}
