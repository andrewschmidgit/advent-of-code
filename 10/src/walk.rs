use crate::{grid::{Grid, Point}, pipe::{Pipe, PIPES_CONNECT_UP, PIPES_CONNECT_LEFT, PIPES_CONNECT_RIGHT, PIPES_CONNECT_DOWN}};

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn get_start_pipe(p: Point, g: &Grid<Pipe>) -> Pipe {
    let mut options = vec![];

    let s = p;
    let w = g.width;
    let h = g.height;

    if s.x > 0 {
        options.push(Point::new(s.x - 1, s.y));
    }

    if s.x < (w - 1) {
        options.push(Point::new(s.x + 1, s.y));
    }

    if s.y > 0 {
        options.push(Point::new(s.x, s.y - 1));
    }

    if s.y < (h - 1) {
        options.push(Point::new(s.x, s.y + 1));
    }

    let options: Vec<_> = options
        .into_iter()
        .filter(|o| {
            g.connects(s, *o)
        })
        .collect();

    let pos_0 = options[0];
    let pos_1 = options[1];

    let x_d_0 = s.x as i32 - pos_0.x as i32;
    let y_d_0 = s.y as i32 - pos_0.y as i32;
    let x_d_1 = s.x as i32 - pos_1.x as i32;
    let y_d_1 = s.y as i32 - pos_1.y as i32;

    let pos_0_connection = match (x_d_0, y_d_0) {
        (0, 1) => Direction::North,
        (0, -1) => Direction::South,
        (-1, 0) => Direction::East,
        (1, 0) => Direction::West,
        _ => panic!("Shouldn't be here"),
    };

    let pos_1_connection = match (x_d_1, y_d_1) {
        (0, 1) => Direction::North,
        (0, -1) => Direction::South,
        (-1, 0) => Direction::East,
        (1, 0) => Direction::West,
        _ => panic!("Shouldn't be here"),
    };

    match (pos_0_connection, pos_1_connection) {
        (Direction::North, Direction::South) |
            (Direction::South, Direction::North) => Pipe::Pipe,

        (Direction::East, Direction::West) |
            (Direction::West, Direction::East) => Pipe::Dash,

        (Direction::North, Direction::East) |
            (Direction::East, Direction::North) => Pipe::NtoE,

        (Direction::North, Direction::West) |
            (Direction::West, Direction::North) => Pipe::WtoN,

        (Direction::South, Direction::East) |
            (Direction::East, Direction::South) => Pipe::EtoS,

        (Direction::South, Direction::West) |
            (Direction::West, Direction::South) => Pipe::WtoS,

        _ => panic!("Shouldn't be here either"),
    }
}

#[derive(Debug)]
pub struct Walker {
    start: Point,
    grid: Grid<Pipe>,
}

impl Walker {
    pub fn new(mut grid: Grid<Pipe>) -> Result<Self, String> {
        let start = grid.find(&Pipe::Start).ok_or("expected start")?;
        let start_pipe = get_start_pipe(start, &grid);
        grid.set(start_pipe, start);

        Ok(Self {
            start,
            grid,
        })
    }

    pub fn find_furthest_point(&self) -> Result<(u32, Point), String> {
        let mut options = vec![];

        let s = self.start;
        let w = self.grid.width;
        let h = self.grid.height;

        if s.x > 0 {
            options.push(Point::new(s.x - 1, s.y));
        }

        if s.x < (w - 1) {
            options.push(Point::new(s.x + 1, s.y));
        }

        if s.y > 0 {
            options.push(Point::new(s.x, s.y - 1));
        }

        if s.y < (h - 1) {
            options.push(Point::new(s.x, s.y + 1));
        }

        let options: Vec<_> = options
            .into_iter()
            .filter(|o| {
                self.grid.connects(s, *o)
            })
            .collect();

        let mut distance = 1;
        let mut distance_grid = Grid::<Option<u32>>::new(w, h);

        let mut left_prev = self.start;
        let mut left = options[0];
        let mut right_prev = self.start;
        let mut right = options[1];

        loop {
            let left_visited = distance_grid.get(left).is_some();
            let right_visited = distance_grid.get(right).is_some();
            if left_visited {
                break Ok((distance - 1, left_prev));
            }
            if right_visited {
                break Ok((distance - 1, right_prev));
            }

            distance += 1;
            *distance_grid.get_mut(left) = Some(distance);
            *distance_grid.get_mut(right) = Some(distance);

            let temp = left;
            left = self.grid.next(left_prev, left).ok_or("expected left to connect")?;
            left_prev = temp;

            let temp = right;
            right = self.grid.next(right_prev, right).ok_or("expected right to connect")?;
            right_prev = temp;
        }
    }

    pub fn get_enclosed_tiles(&self) -> i32 {
        // find the loop
        let path = self.get_path();

        let mut inside_count = 0;
        // look at every tile
        for y in 0..self.grid.height {
            let mut inside = false;
            for x in 0..self.grid.width {
                let point = Point::new(x, y);
                let pipe = *path.get(point);

                match pipe {
                    Pipe::Start => panic!("shouldn't happen"),
                    Pipe::Ground => if inside {
                        inside_count += 1;
                    },
                    Pipe::Pipe |
                    Pipe::NtoE |
                    Pipe::WtoN => inside = !inside,
                    Pipe::Dash |
                    Pipe::WtoS |
                    Pipe::EtoS => (),
                }
            }
        }

        inside_count
    }


    fn get_path(&self) -> Grid<Pipe> {
        let mut options = vec![];

        let s = self.start;
        let w = self.grid.width;
        let h = self.grid.height;

        if s.x > 0 {
            options.push(Point::new(s.x - 1, s.y));
        }

        if s.x < (w - 1) {
            options.push(Point::new(s.x + 1, s.y));
        }

        if s.y > 0 {
            options.push(Point::new(s.x, s.y - 1));
        }

        if s.y < (h - 1) {
            options.push(Point::new(s.x, s.y + 1));
        }

        let start_pipe = self.grid.get(self.start);

        let mut previous = self.start;
        let mut current = match start_pipe {
            Pipe::Pipe => Point::new(s.x, s.y + 1),
            Pipe::Dash => Point::new(s.x + 1, s.y),
            Pipe::NtoE => Point::new(s.x, s.y - 1),
            Pipe::WtoN => Point::new(s.x, s.y - 1),
            Pipe::WtoS => Point::new(s.x, s.y + 1),
            Pipe::EtoS => Point::new(s.x, s.y + 1),
            _ => panic!("Invalid start pipe"),
        };

        let mut path = Grid::new(self.grid.width, self.grid.height);

        loop {
            path.set(*self.grid.get(current), current);

            if current == self.start { break; }

            let temp = current;
            current = self.grid.next(previous, current).expect("expected right to connect");
            previous = temp;
        }

        path
    }
}

impl Grid<Pipe> {
    pub fn next(&self, from: Point, current: Point) -> Option<Point> {
        let pipe = self.get(current);
        let (x_t, y_t) = current.distance_from(&from);

        match pipe {
            Pipe::Start => Some(current),
            Pipe::Ground => None,

            Pipe::Pipe => Some(Point::from_p(current, 0, y_t)),
            Pipe::Dash => Some(Point::from_p(current, x_t, 0)),

            Pipe::NtoE => Some(Point::from_p(current, x_t + 1, y_t - 1)),
            Pipe::WtoN => Some(Point::from_p(current, x_t - 1, y_t - 1)),
            Pipe::WtoS => Some(Point::from_p(current, x_t - 1, y_t + 1)),
            Pipe::EtoS => Some(Point::from_p(current, x_t + 1, y_t + 1)),
        }
    }

    pub fn connects(&self, from: Point, to: Point) -> bool {
        let (x_t, y_t) = to.distance_from(&from);

        let from_pipe = self.get(from);
        let to_pipe = self.get(to);

        if x_t == -1 && y_t == 0 {
            PIPES_CONNECT_LEFT.contains(from_pipe) && PIPES_CONNECT_RIGHT.contains(to_pipe)
        } else if x_t == 1 && y_t == 0 {
            PIPES_CONNECT_LEFT.contains(to_pipe) && PIPES_CONNECT_RIGHT.contains(from_pipe)
        } else if x_t == 0 && y_t == -1 {
            PIPES_CONNECT_UP.contains(from_pipe) && PIPES_CONNECT_DOWN.contains(to_pipe)
        } else if x_t == 0 && y_t == 1 {
            PIPES_CONNECT_UP.contains(to_pipe) && PIPES_CONNECT_DOWN.contains(from_pipe)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::build_grid;

    use super::*;

    #[test]
    fn grid_next() {
        let grid = build_grid(
r".....
.F-7.
.|.|.
.L-J.
.....");

        let exps = [
            // Pipe
            (Point::new(1, 3), grid.next(Point::new(1, 1), Point::new(1, 2))),
            (Point::new(1, 1), grid.next(Point::new(1, 3), Point::new(1, 2))),

            // Dash
            (Point::new(3, 1), grid.next(Point::new(1, 1), Point::new(2, 1))),
            (Point::new(1, 1), grid.next(Point::new(3, 1), Point::new(2, 1))),

            // NtoE
            (Point::new(2, 3), grid.next(Point::new(1, 2), Point::new(1, 3))),
            (Point::new(1, 2), grid.next(Point::new(2, 3), Point::new(1, 3))),

            // WtoN
            (Point::new(3, 2), grid.next(Point::new(2, 3), Point::new(3, 3))),
            (Point::new(2, 3), grid.next(Point::new(3, 2), Point::new(3, 3))),
        ];

        for (e, p) in exps.into_iter() {
            assert_eq!(e, p.unwrap());
        }
    }

    #[test]
    fn grid_connects() {
        let grid = build_grid(
r".....
.F-7.
.|.|.
.L-J.
.....");

        let exps = [
            (Point::new(1, 1), Point::new(2, 1)),
            (Point::new(2, 1), Point::new(1, 1)),
        ];

        for (from, to) in exps.into_iter() {
            assert!(grid.connects(from, to), "{} -> {} did not connect", from, to);
        }
    }

    #[test]
    fn grid_finds_enclosed() {
        let grid = build_grid("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");

        let walker = Walker::new(grid).unwrap();

        assert_eq!(walker.get_enclosed_tiles(), 4);
    }
}
