use std::str::FromStr;
use crate::range::Range;

#[derive(Debug, PartialEq)]
pub struct Map {
    pub title: String,
    ranges: Vec<Range>,
}

impl Map {
    pub fn map(&self, value: &u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(v) = range.map(value) {
                return v;
            }
        }

        *value
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let title = lines.next().ok_or("Should have a title line")?.to_owned();

        let maps = lines
            .filter_map(|l| l.parse().ok())
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
                "1 2 3".parse().unwrap(),
                "4 5 6".parse().unwrap(),
                "10 11 1".parse().unwrap(),
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
