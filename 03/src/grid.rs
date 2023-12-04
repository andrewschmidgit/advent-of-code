use std::{str::FromStr, collections::VecDeque, ops::RangeInclusive};

use crate::{element::Element, point::Point};

#[derive(Debug, PartialEq)]
pub struct Grid {
    array: Vec<Vec<Element>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn find_symbol_positions(&self) -> Vec<(char, Point)> {
        let mut pos = vec![];

        for (y, row) in self.array.iter().enumerate() {
            for (x, el) in row.iter().enumerate() {
                if let Element::Symbol(c) = el {
                    pos.push((*c, Point::new(x, y)));
                }
            }
        }

        pos
    }

    pub fn find_part_numbers(&self, p: Point) -> Vec<u32> {
        let mut part_numbers = vec![];

        // there is an assumption that the neighbor positions are ordered
        let neighbors = p.get_neighbor_positions(self.width, self.height);
        let mut visited: Vec<&Point> = vec![];

        for (i, n) in neighbors.iter().enumerate() {
            if visited.contains(&n) { 
                continue;
            }

            let e = self.get(n);
            if let Some(Element::Number(_)) = e {
                // find beginning and end cursor of the number
                let cursors = self.find_number_cursors(n);
                let row = &self.array[n.y];

                let digit_elements = &row[cursors.clone()];

                let digits: Vec<_> = digit_elements
                    .iter()
                    .filter_map(|e| {
                        if let Element::Number(n) = e { return Some(*n) }
                        None
                    })
                    .collect();

                // need to update visited as needed
                for o in neighbors.iter() {
                    let on_same_row = o.y == n.y;
                    let within_cursor_range = cursors.contains(&o.x);
                    if on_same_row && within_cursor_range {
                        visited.push(o);
                    }
                }

                let part_number = digits_to_number(&digits);
                part_numbers.push(part_number);
            }
        }

        part_numbers
    }

    fn get(&self, p: &Point) -> Option<Element> {
        if p.x < self.width && p.y < self.height {
            return Some(self.array[p.y][p.x]);
        }

        None
    }

    fn find_number_cursors(&self, p: &Point) -> RangeInclusive<usize> {
        let mut start_cursor = p.x;
        let mut end_cursor = p.x;

        let row = &self.array[p.y];

        loop {
            if start_cursor == 0 {
                break;
            }

            if let Element::Number(_) = row[start_cursor - 1] {
                start_cursor -= 1;
            } else {
                break;
            }
        }

        loop {
            if end_cursor == row.len() - 1 {
                break;
            }

            if let Element::Number(_) = row[end_cursor + 1] {
                end_cursor += 1;
            } else {
                break;
            }
        }

        start_cursor..=end_cursor
    }
}

fn digits_to_number(ds: &[u32]) -> u32 {
    let mut place = ds.len() as u32;
    let mut value = 0;

    for d in ds {
        place -= 1;

        value += *d * 10u32.pow(place);
    }

    value
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let height = s.lines().count();
        let mut array = vec![vec![Element::None; width]; height];

        for (y, line) in s.lines().enumerate() {
            for (x, el) in line.chars().enumerate() {
                array[y][x] = el.into();
            }
        }

        Ok(Self {
            array,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_parses_single_row() {
        let input = ".";
        let exp = Grid {
            array: vec![vec![Element::None]],
            width: 1,
            height: 1,
        };

        let grid: Grid = input.parse().unwrap();

        assert_eq!(exp, grid, "Grids should be equal");
    }

    #[test]
    fn grid_parses_multi_row() {
        let input = ".\n.";
        let exp = Grid {
            array: vec![
                vec![Element::None],
                vec![Element::None],
            ],
            width: 1,
            height: 2,
        };

        let grid: Grid = input.parse().unwrap();

        assert_eq!(exp, grid, "Grids should be equal");
    }

    #[test]
    fn grid_gets_symbols() {
        let input = "_";
        let grid: Grid = input.parse().unwrap();
        let exp = vec![('_', Point::new(0, 0))];

        let positions = grid.find_symbol_positions();

        assert_eq!(exp, positions, "Should've found the symbol");
    }

    #[test]
    fn grid_can_get_el_by_point() {
        let grid: Grid = ".".parse().unwrap();
        let exp = Element::None;

        let point = grid.get(&Point { x: 0, y: 0 }).unwrap();

        assert_eq!(exp, point);
    }

    #[test]
    fn grid_digits_to_number_converts_successfully() {
        let exp = 123;

        let number = digits_to_number(&[1, 2, 3]);

        assert_eq!(exp, number);
    }

    #[test]
    fn grid_finds_number_cursors_given_a_point() {
        let grid: Grid = "...123...".parse().unwrap();
        let point = Point::new(4, 0);
        let exp = 3..=5;

        let cursors = grid.find_number_cursors(&point);

        assert_eq!(exp, cursors);
    }

    #[test]
    fn grid_finds_part_numbers_simple() {
        let input = 
r"1.2
3$4
5.6";
        let point = Point::new(1, 1);
        let grid: Grid = input.parse().unwrap();
        let exp = vec![1, 2, 3, 4, 5, 6];

        let part_numbers = grid.find_part_numbers(point);

        assert_eq!(exp, part_numbers);
    }

    #[test]
    fn o_grid_finds_part_numbers_complex() {
        let input = 
r"11.
3$4
556";
        let point = Point::new(1, 1);
        let grid: Grid = input.parse().unwrap();
        let exp = vec![11, 3, 4, 556];

        let part_numbers = grid.find_part_numbers(point);

        assert_eq!(exp, part_numbers);
    }
}
