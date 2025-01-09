use std::{
    collections::{HashSet, VecDeque},
    env, fs,
};

type Point = (i32, i32);

fn main() {
    let filename = env::args().skip(1).next().expect("filename");
    let input = fs::read_to_string(&filename).expect("file");

    let mut visited: HashSet<Point> = HashSet::new();
    let grid = parse(&input);

    let mut regions = vec![];

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = (x as i32, y as i32);

            if visited.contains(&p) {
                continue;
            }

            let region = find_region(&grid, p).unwrap();
            visited.extend(&region.points);
            regions.push(region);
        }
    }

    let total: usize = regions.iter().map(|r| r.cost()).sum();

    println!("total cost for {}: {}", filename, total);
}

fn parse(input: &str) -> Grid {
    let chars = input.lines().flat_map(|l| l.chars()).collect();

    Grid {
        width: input.lines().next().unwrap_or_default().len(),
        height: input.lines().count(),
        cells: chars,
    }
}

fn find_region(grid: &Grid, start_at: Point) -> Option<Region> {
    let start = grid.get_at(start_at)?;
    let mut region = Region::new(*start, start_at);
    let mut queue = VecDeque::new();

    queue.push_back((start_at.0 + 1, start_at.1));
    queue.push_back((start_at.0, start_at.1 + 1));
    queue.push_back((start_at.0 - 1, start_at.1));
    queue.push_back((start_at.0, start_at.1 - 1));

    while let Some(p) = queue.pop_front() {
        let Some(c) = grid.get_at(p) else {
            continue;
        };

        // Different region
        if *c != region.key {
            continue;
        }

        // Already been here, don't cycle
        if !region.points.insert(p) {
            continue;
        }

        queue.push_back((p.0 + 1, p.1));
        queue.push_back((p.0, p.1 + 1));
        queue.push_back((p.0 - 1, p.1));
        queue.push_back((p.0, p.1 - 1));
    }

    Some(region)
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

impl Grid {
    pub fn get_at(&self, p: Point) -> Option<&char> {
        let (x, y) = p;

        if x < 0 || y < 0 {
            return None;
        }

        if x as usize >= self.width || y as usize >= self.height {
            return None;
        }

        let index = y * self.width as i32 + x;
        self.cells.get(index as usize)
    }
}

#[derive(Debug)]
struct Region {
    key: char,
    points: HashSet<Point>,
}

impl Region {
    pub fn new(key: char, point: Point) -> Self {
        Self {
            key,
            points: HashSet::from([point]),
        }
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> usize {
        let mut sides = 0;

        for p in self.points.iter() {
            sides += [
                (p.0 - 1, p.1),
                (p.0 + 1, p.1),
                (p.0, p.1 - 1),
                (p.0, p.1 + 1),
            ]
            .iter()
            .filter(|p| !self.points.contains(p))
            .count();
        }

        sides
    }

    pub fn cost(&self) -> usize {
        self.area() * self.perimeter()
    }
}

fn display_region(grid: &Grid, region: &Region) -> String {
    let mut s = String::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            let c = if region.points.contains(&(x as i32, y as i32)) {
                region.key
            } else {
                '.'
            };

            s.push(c);
        }

        if y != grid.height - 1 {
            s.push('\n');
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn region_loop() {
        let g = parse(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        let r = find_region(&g, (0, 0)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "OOOOO
O.O.O
OOOOO
O.O.O
OOOOO",
        );
    }
    #[test]
    fn finds_region() {
        let g = parse(
            "AAAA
BBCD
BBCC
EEEC",
        );

        let r = find_region(&g, (0, 0)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "AAAA
....
....
...."
        );

        let r = find_region(&g, (1, 1)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "....
BB..
BB..
...."
        );

        let r = find_region(&g, (2, 2)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "....
..C.
..CC
...C"
        );

        let r = find_region(&g, (3, 1)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "....
...D
....
...."
        );

        let r = find_region(&g, (1, 3)).unwrap();
        assert_eq!(
            display_region(&g, &r),
            "....
....
....
EEE."
        );
    }

    #[test]
    fn region_perimeter() {
        let g = parse(
            "AAAA
BBCD
BBCC
EEEC",
        );

        let r = find_region(&g, (0, 0)).unwrap();
        assert_eq!(r.perimeter(), 10);
        let r = find_region(&g, (1, 1)).unwrap();
        assert_eq!(r.perimeter(), 8);
        let r = find_region(&g, (2, 2)).unwrap();
        assert_eq!(r.perimeter(), 10);
        let r = find_region(&g, (3, 1)).unwrap();
        assert_eq!(r.perimeter(), 4);
        let r = find_region(&g, (1, 3)).unwrap();
        assert_eq!(r.perimeter(), 8);

        let g = parse(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );

        let r = find_region(&g, (0, 0)).unwrap();
        let mut points = r.points.iter().collect::<Vec<_>>();

        points.sort_by(|(ax, ay), (bx, by)| {
            let x_cmp = ax.cmp(bx);
            if x_cmp == Ordering::Equal {
                return ay.cmp(by);
            } else {
                x_cmp
            }
        });

        assert_eq!(r.perimeter(), 36);
    }
}
