use std::{collections::HashMap, str::FromStr, slice::Iter};

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("Expected 'L' or 'R'".into())
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    pub directions: Vec<Direction>,
    pub nodes: HashMap<String, (String, String)>
}

impl Map {
    pub fn directions(&self) -> std::iter::Cycle<Iter<'_, Direction>> {
        self.directions.iter().cycle()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let directions = lines.next().ok_or("Expected directions")?;
        lines.next().ok_or("Expected blank line")?;

        let mut nodes = HashMap::new();
        for line in lines {
            let (src, children) = parse_node(line)?;
            nodes.insert(src, children);
        }

        Ok(Self {
            directions: directions
                .chars()
                .filter_map(|c| Direction::from_char(c).ok())
                .collect(),
            nodes,
        })
    }
}

fn parse_node(s: &str) -> Result<(String, (String, String)), String> {
    let s: String = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let (src, dest) = s
        .split_once('=')
        .ok_or("Expected '='")?;

    let s = dest.chars()
        .filter(|c| c == &',' || c.is_alphanumeric())
        .collect::<String>();

    let (left, right) = s
        .split_once(',')
        .ok_or("Expected comma")?;

    Ok((src.into(), (left.into(), right.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_parses() {
        let s = "AAA = (BBB, CCC)";
        let exp = ("AAA".into(), ("BBB".into(), "CCC".into()));

        let parsed = parse_node(s).unwrap();

        assert_eq!(exp, parsed);
    }

    #[test]
    fn map_parses() {
        let mut nodes = HashMap::new();
        nodes.insert("AAA".into(), ("BBB".into(), "CCC".into()));
        nodes.insert("BBB".into(), ("DDD".into(), "EEE".into()));
        let exp = Map {
            directions: vec![Direction::Right, Direction::Left],
            nodes,
        };
        let s = 
r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)";

        let result = s.parse().unwrap();

        assert_eq!(exp, result);
    }
}
