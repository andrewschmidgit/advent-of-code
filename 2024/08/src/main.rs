use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fs,
    str::FromStr,
    usize,
};

fn main() -> Result<(), Box<dyn Error>> {
    let filename = env::args().skip(1).next().expect("should provide filename");
    let input = fs::read_to_string(&filename).expect("should be a real file");

    let grid: Grid = input.parse()?;

    // part 1
    let antinodes = grid.find_antinodes(Some(1));
    let unique_pos = antinodes
        .iter()
        .flat_map(|(_, v)| v)
        .collect::<HashSet<_>>()
        .len();

    println!(
        "antinode location count: {}, unique_positions: {}",
        antinodes.iter().map(|(_, v)| v.len()).sum::<usize>(),
        unique_pos
    );

    // part 2
    let antinodes = grid.find_antinodes(None);
    let unique_pos = antinodes
        .iter()
        .flat_map(|(_, v)| v)
        .collect::<HashSet<_>>()
        .len();

    println!(
        "antinode location count: {}, unique_positions: {}",
        antinodes.iter().map(|(_, v)| v.len()).sum::<usize>(),
        unique_pos
    );

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub c: char,
}

impl Point {
    pub fn new(x: usize, y: usize, c: char) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
            c,
        }
    }

    fn next_along_vector(&self, dx: i32, dy: i32, c: char) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            c,
        }
    }

    pub fn get_antinodes(
        &self,
        other: &Self,
        x_bound: usize,
        y_bound: usize,
        limit: Option<usize>,
    ) -> Vec<Self> {
        let in_bounds = |p: &Point| {
            let x_good = 0 <= p.x && p.x < x_bound as i32;
            let y_good = 0 <= p.y && p.y < y_bound as i32;

            x_good && y_good
        };

        let (dx, dy) = (other.x - self.x, other.y - self.y);

        let mut nodes = vec![];

        // traverse line forwards
        let mut current_node = *other;
        let mut count = 0;
        loop {
            if let Some(limit) = limit {
                if count >= limit {
                    break;
                }
            }

            count += 1;
            current_node = current_node.next_along_vector(dx, dy, '#');

            if !in_bounds(&current_node) {
                break;
            }

            nodes.push(current_node);
        }

        // traverse line backwards
        let (dx, dy) = (-dx, -dy);
        let mut current_node = self.clone();
        let mut count = 0;
        loop {
            if let Some(limit) = limit {
                if count >= limit {
                    break;
                }
            }

            count += 1;
            current_node = current_node.next_along_vector(dx, dy, '#');

            if !in_bounds(&current_node) {
                break;
            }

            nodes.push(current_node);
        }

        nodes
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    list: Vec<Point>,
}

impl Grid {
    pub fn find_antinodes(&self, limit: Option<usize>) -> HashMap<char, HashSet<Point>> {
        let antennae_by_freq =
            self.list
                .iter()
                .filter(|p| p.c != '.')
                .fold(HashMap::new(), |mut acc, p| {
                    acc.entry(p.c).or_insert(vec![]).push(p);
                    acc
                });

        let mut res_freqs_by_freq = HashMap::new();
        for (f, ps) in antennae_by_freq {
            for i in 0..ps.len() {
                for j in (i + 1)..ps.len() {
                    let a = ps[i];
                    let b = ps[j];

                    let mut nodes = a.get_antinodes(b, self.width, self.height, limit);

                    if limit.is_none() {
                        nodes.push(Point {
                            x: a.x,
                            y: a.y,
                            c: '#',
                        });
                        nodes.push(Point {
                            x: b.x,
                            y: b.y,
                            c: '#',
                        });
                    }

                    for n in nodes {
                        res_freqs_by_freq
                            .entry(f)
                            .or_insert(HashSet::new())
                            .insert(n);
                    }
                }
            }
        }

        res_freqs_by_freq
    }
}

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or("no rows")?.len();
        let list = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| Point::new(x, y, c))
            })
            .collect();

        Ok(Self {
            width,
            height,
            list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_news() {
        let grid: Grid = "...
a..
..b"
        .parse()
        .unwrap();

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(
            grid.list,
            vec![
                Point::new(0, 0, '.'),
                Point::new(1, 0, '.'),
                Point::new(2, 0, '.'),
                Point::new(0, 1, 'a'),
                Point::new(1, 1, '.'),
                Point::new(2, 1, '.'),
                Point::new(0, 2, '.'),
                Point::new(1, 2, '.'),
                Point::new(2, 2, 'b'),
            ]
        );
    }

    #[test]
    fn grid_finds_antinodes() {
        let exp = HashMap::from([(
            'a',
            HashSet::from([Point::new(6, 7, '#'), Point::new(3, 1, '#')]),
        )]);

        let grid: Grid = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            .parse()
            .unwrap();

        let antinodes = grid.find_antinodes(Some(1));

        assert_eq!(antinodes, exp);
    }

    #[test]
    fn grid_finds_unlimited_antinodes() {
        let grid: Grid = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."
            .parse()
            .unwrap();

        let antinodes = grid.find_antinodes(None);
        assert_eq!(antinodes[&'T'].len(), 9);
    }
}
