use std::collections::HashMap;
use std::{env, error::Error};
use std::fs;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("filename plz");
    let contents = fs::read_to_string(filename)?;

    let parsed = parse(&contents)?;

    let path = traverse(parsed, "ZZZ")?;

    println!("steps: {}", path.len());

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Result<Self, String> {
        if c == 'L' {
            return Ok(Self::Left);
        } else if c == 'R' {
            return Ok(Self::Right);
        }

        Err("Could not translate character".into())
    }
}

#[derive(Debug, PartialEq)]
struct ParseResult {
    directions: Vec<Direction>,
    graph: HashMap<String, (String, String)>
}

fn parse(s: &str) -> Result<ParseResult, String> {
    let mut lines = s.lines();
    let directions = lines.next().ok_or("Expected directions")?;
    lines.next().ok_or("Expected blank line")?;

    let mut graph = HashMap::new();
    for line in lines {
        let (src, children) = parse_node(line)?;
        graph.insert(src, children);
    }

    Ok(ParseResult {
        directions: directions
            .chars()
            .filter_map(|c| Direction::from_char(c).ok())
            .collect(),
        graph,
    })
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
        .filter(|c| c == &',' || c.is_alphabetic())
        .collect::<String>();

    let (left, right) = s
        .split_once(',')
        .ok_or("Expected comma")?;

    Ok((src.into(), (left.into(), right.into())))
}

fn traverse(p: ParseResult, target: &str) -> Result<Vec<String>, String> {
    let mut res = vec![];
    let mut found = false;

    let mut key = "AAA";
    loop {
        if found { break; }

        for d in p.directions.iter() {
            res.push(key.into());

            let node = p.graph.get(key).ok_or("Expected key")?;
            key = match d {
                Direction::Left => &node.0,
                Direction::Right => &node.1,
            };
        }

        found = key == target;
    }

    Ok(res)
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
    fn parse_parses() {
        let mut graph = HashMap::new();
        graph.insert("AAA".into(), ("BBB".into(), "CCC".into()));
        graph.insert("BBB".into(), ("DDD".into(), "EEE".into()));
        let exp = ParseResult {
            directions: vec![Direction::Right, Direction::Left],
            graph,
        };
        let s = 
r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)";

        let result = parse(s).unwrap();

        assert_eq!(exp, result);
    }

    #[test]
    fn traverse_traverses_single() {
        let s = 
r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let p = parse(s).unwrap();

        let path = traverse(p, "ZZZ").unwrap();

        assert_eq!(2, path.len());
    }

    #[test]
    fn traverse_traverses_multi() {
        let s = 
r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let p = parse(s).unwrap();

        let path = traverse(p, "ZZZ").unwrap();

        assert_eq!(6, path.len());
    }
}
