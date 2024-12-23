use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    str::FromStr,
};

const MAX_HEIGHT: u32 = 9;

#[derive(Debug)]
pub enum MapError {
    TrailheadNotFound,
}

#[derive(Debug)]
pub struct Map {
    heights_by_position: HashMap<u32, Vec<Point>>,
}

impl Map {
    pub fn score(&self) -> Result<usize, MapError> {
        let trailheads = self
            .heights_by_position
            .get(&0)
            .ok_or(MapError::TrailheadNotFound)?;

        let mut queue = VecDeque::new();
        let mut score = 0;

        for th in trailheads {
            queue.push_back((th, 0));

            let mut visited_ends = HashSet::new();

            while let Some((point, height)) = queue.pop_front() {
                let next_height = height + 1;

                let Some(points) = self.heights_by_position.get(&next_height) else {
                    continue;
                };

                let valid_points: Vec<_> = points
                    .into_iter()
                    .filter_map(|p| {
                        if p.is_next_to(point) {
                            Some((p, next_height))
                        } else {
                            None
                        }
                    })
                    .collect();

                for vp in valid_points {
                    if next_height == MAX_HEIGHT && visited_ends.insert(vp) {
                        score += 1;
                        continue;
                    }
                    queue.push_back(vp);
                }
            }
        }

        Ok(score)
    }
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights_by_position = HashMap::new();

        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if let Some(n) = c.to_digit(10) {
                    heights_by_position
                        .entry(n)
                        .or_insert(Vec::new())
                        .push(Point::new(x, y));
                }
            }
        }

        Ok(Self {
            heights_by_position,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn is_next_to(&self, other: &Self) -> bool {
        let transforms = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        transforms.into_iter().any(|(dx, dy)| {
            let new_x = self.x as i32 + dx;
            let new_y = self.y as i32 + dy;

            new_x == other.x as i32 && new_y == other.y as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_parses() {
        let map: Map = "90.9
.198"
            .parse()
            .unwrap();

        assert_eq!(map.heights_by_position[&0], vec![(1, 0)]);
        assert_eq!(map.heights_by_position[&1], vec![(1, 1)]);
        assert_eq!(map.heights_by_position[&8], vec![(3, 1)]);
        assert_eq!(map.heights_by_position[&9], vec![(0, 0), (3, 0), (2, 1)]);

        assert_eq!(map.heights_by_position.len(), 4)
    }

    #[test]
    fn map_scores_0() {
        let map: Map = "0123
1234
8765
9876"
            .parse()
            .unwrap();
        let score = map.score().unwrap();
        assert_eq!(score, 1);
    }

    #[test]
    fn map_scores_1() {
        let map: Map = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
            .parse()
            .unwrap();
        let score = map.score().unwrap();
        assert_eq!(score, 2);
    }

    #[test]
    fn map_scores_2() {
        let map: Map = "..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            .parse()
            .unwrap();
        let score = map.score().unwrap();
        assert_eq!(score, 4);
    }

    #[test]
    fn map_scores_3() {
        let map: Map = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"
            .parse()
            .unwrap();
        let score = map.score().unwrap();
        assert_eq!(score, 3);
    }

    #[test]
    fn map_scores_4() {
        let map: Map = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .parse()
            .unwrap();
        let score = map.score().unwrap();
        assert_eq!(score, 36);
    }
}
