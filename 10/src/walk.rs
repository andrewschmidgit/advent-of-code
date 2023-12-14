use crate::{grid::{Grid, Point}, pipe::{Pipe, PIPES_CONNECT_UP, PIPES_CONNECT_LEFT, PIPES_CONNECT_RIGHT, PIPES_CONNECT_DOWN}};

#[derive(Debug)]
pub struct Walker {
    start: Point,
    grid: Grid<Pipe>,
}

impl Walker {
    pub fn new(grid: Grid<Pipe>) -> Result<Self, String> {
        let start = grid.find(&Pipe::Start).ok_or("expected start")?;

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
}
