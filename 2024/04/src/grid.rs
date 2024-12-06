use std::{fmt::Debug, str::FromStr};

use crate::{pattern::Pattern, point::Point, vector::Vector};

const DIRECTIONS: [Vector; 8] = [
    Vector { x: -1, y: -1 },
    Vector { x: 00, y: -1 },
    Vector { x: 01, y: -1 },
    Vector { x: 01, y: 00 },
    Vector { x: 01, y: 01 },
    Vector { x: 00, y: 01 },
    Vector { x: -1, y: 01 },
    Vector { x: -1, y: 00 },
];

#[derive(Debug)]
pub struct Grid<T> {
    array: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl FromStr for Grid<char> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().ok_or("")?.chars().count();

        if s.lines().any(|l| l.chars().count() != width) {
            return Err("all lines weren't the same length".to_string());
        }

        let array: Vec<_> = s.chars().filter(|c| !c.is_whitespace()).collect();

        if array.len() != height * width {
            return Err("array length not equal to height * width".to_string());
        }

        Ok(Grid {
            array,
            width,
            height,
        })
    }
}

impl<T> Grid<T>
where
    T: PartialEq + Debug,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            array: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    fn get(self: &Self, point: &Point) -> Option<&T> {
        self.array.get(point.y * self.width + point.x)
    }

    fn find_target_in_direction(
        self: &Self,
        target: &T,
        p_start: &Point,
        direction: &Vector,
    ) -> Option<(&T, Point)> {
        let p = p_start.transform(direction).ok()?;

        if p.x >= self.width || p.y >= self.height {
            return None;
        }

        let el = self.array.get(p.y * self.width + p.x)?;
        if target != el {
            return None;
        }

        Some((el, p))
    }

    fn get_in_direction(
        self: &Self,
        p_start: &Point,
        v: &Vector,
        max_length: usize,
    ) -> Option<Vec<Point>> {
        let mut p = p_start.clone();
        let mut points = vec![p_start.clone()];

        for _ in 1..max_length {
            p = p.transform(v).ok()?;

            if p.x >= self.width || p.y >= self.height {
                return None;
            }

            points.push(p.clone());
        }

        Some(points)
    }

    pub fn search(self: &Self, query: &[T]) -> Option<Vec<Vec<(Point, &T)>>> {
        let q_start = query.get(0)?;
        let q_length = query.len();

        // for every letter
        // if that letter == q_start
        // check all directions for the query

        let mut result: Vec<Vec<(Point, &T)>> = Vec::new();

        for (i, value) in self.array.iter().enumerate() {
            if value != q_start {
                continue;
            }

            let current_point = Point {
                x: i % self.width,
                y: i / self.width,
            };

            let mut sequences: Vec<_> = DIRECTIONS
                .iter()
                .filter_map(|d| self.get_in_direction(&current_point, &d, q_length))
                .map(|s| {
                    s.iter()
                        .filter_map(|p| self.get(p).map(|v| (p.clone(), v)))
                        .collect::<Vec<(Point, &T)>>()
                })
                .filter(|s| s.iter().zip(query).all(|((_, a), b)| **a == *b))
                .collect();

            result.append(&mut sequences);
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl Grid<char> {
    pub fn find(self: &Self, pattern: &Pattern) -> Option<Vec<usize>> {
        let (offset, char) = pattern.get_start()?;

        let positions: Vec<_> = self
            .array
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if *c == char && i >= offset {
                    Some(i - offset)
                } else {
                    None
                }
            })
            .collect();

        // 0 1 2
        // 3 4 5
        // 6 7 8
        // starting at 1, width: 3, sub_width: 2, sub_height: 2, I want 1 2 4 5
        // i % width >= 1 && i % width < 1 + sub_height

        let mut matches = vec![];

        for p in positions {
            let chars: Vec<_> = self
                .array
                .iter()
                .enumerate()
                .filter_map(|(i, c)| {
                    let i_row_pos = i % self.width;
                    let p_row_pos = p % self.width;
                    let i_col_pos = i / self.width;
                    let p_col_pos = p / self.width;

                    let p_x_too_small = i_row_pos < p_row_pos;
                    let p_x_too_large = i_row_pos >= p_row_pos + pattern.width;
                    let p_y_too_small = i_col_pos < p_col_pos;
                    let p_y_too_large = i_col_pos >= p_col_pos + pattern.len() / pattern.width;
                    if p_x_too_small || p_x_too_large || p_y_too_small || p_y_too_large {
                        return None;
                    }

                    Some(*c)
                })
                .collect();

            if pattern == chars.as_slice() {
                matches.push(p);
            }
        }

        if matches.len() > 0 {
            Some(matches)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_initializes() {
        let width = 3;
        let height = 4;
        let grid: Grid<i32> = Grid::new(width, height);

        assert_eq!(grid.width, width);
        assert_eq!(grid.height, height);
        assert_eq!(grid.array.capacity(), width * height);
    }

    #[test]
    fn grid_parses() {
        let grid: Grid<char> = "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            .parse()
            .unwrap();

        assert_eq!(grid.width, 6);
        assert_eq!(grid.height, 5);
        assert_eq!(
            grid.array,
            vec![
                '.', '.', 'X', '.', '.', '.', '.', 'S', 'A', 'M', 'X', '.', '.', 'A', '.', '.',
                'A', '.', 'X', 'M', 'A', 'S', '.', 'S', '.', 'X', '.', '.', '.', '.'
            ]
        );
    }

    #[test]
    fn grid_finds_target_in_direction() {
        let grid = Grid {
            width: 2,
            height: 2,
            array: vec!['a', 'b', 'c', 'd'],
        };

        let cases = vec![
            ('a', Point { x: 0, y: 0 }, Vector { x: 0, y: -1 }, None),
            ('a', Point { x: 0, y: 0 }, Vector { x: -1, y: 0 }, None),
            ('a', Point { x: 0, y: 0 }, Vector { x: -1, y: -1 }, None),
            ('a', Point { x: 1, y: 1 }, Vector { x: 1, y: 0 }, None),
            ('a', Point { x: 1, y: 1 }, Vector { x: 0, y: 1 }, None),
            ('a', Point { x: 1, y: 1 }, Vector { x: 1, y: 1 }, None),
            ('a', Point { x: 0, y: 0 }, Vector { x: 1, y: 1 }, None),
            ('d', Point { x: 1, y: 1 }, Vector { x: -1, y: -1 }, None),
            (
                'd',
                Point { x: 0, y: 0 },
                Vector { x: 1, y: 1 },
                Some((&'d', Point { x: 1, y: 1 })),
            ),
            (
                'a',
                Point { x: 1, y: 1 },
                Vector { x: -1, y: -1 },
                Some((&'a', Point { x: 0, y: 0 })),
            ),
        ];

        for (target, p_start, v, e) in cases {
            let p = grid.find_target_in_direction(&target, &p_start, &v);
            assert_eq!(p, e);
        }
    }

    #[test]
    fn grid_gets_points_in_direction() {
        let grid: Grid<i32> = Grid::new(3, 3);
        let cases = vec![
            (
                3,
                Point::new(0, 0),
                Vector { x: 1, y: 1 },
                Some(vec![Point::new(0, 0), Point::new(1, 1), Point::new(2, 2)]),
            ),
            (
                2,
                Point::new(1, 1),
                Vector { x: 1, y: 1 },
                Some(vec![Point::new(1, 1), Point::new(2, 2)]),
            ),
            (
                3,
                Point::new(2, 0),
                Vector { x: 0, y: 1 },
                Some(vec![Point::new(2, 0), Point::new(2, 1), Point::new(2, 2)]),
            ),
        ];

        for (length, start, v, e) in cases {
            let points = grid.get_in_direction(&start, &v, length);
            assert_eq!(points, e);
        }
    }

    #[test]
    fn grid_searches() {
        let grid: Grid<char> = "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            .parse()
            .unwrap();

        let search_text: Vec<_> = "XMAS".chars().collect();
        let search_result = grid.search(&search_text).unwrap();

        assert_eq!(search_result.len(), 4);

        for sequence in search_result {
            let found_chars: Vec<_> = sequence.iter().map(|(_, c)| **c).collect();
            assert_eq!(search_text, found_chars);
        }
    }

    #[test]
    fn grid_findx() {
        let grid: Grid<char> = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            .parse()
            .unwrap();

        let pattern: Pattern = "M.S
.A.
M.S"
        .parse()
        .unwrap();

        let matching_indexes = grid.find(&pattern).unwrap();

        assert_eq!(matching_indexes, vec![1, 21]);
    }
}
