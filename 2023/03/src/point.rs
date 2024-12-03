use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn get_neighbor_positions(&self, x_bound: usize, y_bound: usize) -> Vec<Self> {
        let mut vec = Vec::new();

        // y above
        if self.y != 0 {
            if self.x != 0 {
                vec.push(Point { x: self.x - 1, y: self.y - 1 });
            }

            vec.push(Point { x: self.x, y: self.y - 1 });

            if self.x + 1 != x_bound {
                vec.push(Point { x: self.x + 1, y: self.y - 1 });
            }
        }

        // current y
        if self.x != 0 {
            vec.push(Point { x: self.x - 1, y: self.y });
        }

        if self.x + 1 != x_bound {
            vec.push(Point { x: self.x + 1, y: self.y });
        }

        // y below
        if self.y + 1 != y_bound {
            if self.x != 0 {
                vec.push(Point { x: self.x - 1, y: self.y + 1 });
            }

            vec.push(Point { x: self.x, y: self.y + 1 });

            if self.x + 1 != x_bound {
                vec.push(Point { x: self.x + 1, y: self.y + 1 });
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn point_neighbor_positons_gets_all() {
        let x_bound = 3;
        let y_bound = 3;
        let point = Point::new(1, 1);
        let exp = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),

            Point::new(0, 1),
            Point::new(2, 1),

            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(2, 2),
        ];

        let neighbors = point.get_neighbor_positions(x_bound, y_bound);

        assert_eq!(exp, neighbors);
    }

    #[test]
    fn point_neighbor_positons_respects_x_lower_bound() {
        let x_bound = 3;
        let y_bound = 3;
        let point = Point::new(0, 1);
        let exp = vec![
            Point::new(0, 0),
            Point::new(1, 0),

            Point::new(1, 1),

            Point::new(0, 2),
            Point::new(1, 2),
        ];

        let neighbors = point.get_neighbor_positions(x_bound, y_bound);

        assert_eq!(exp, neighbors);
    }

    #[test]
    fn point_neighbor_positons_respects_x_upper_bound() {
        let x_bound = 3;
        let y_bound = 3;
        let point = Point::new(2, 1);
        let exp = vec![
            Point::new(1, 0),
            Point::new(2, 0),

            Point::new(1, 1),

            Point::new(1, 2),
            Point::new(2, 2),
        ];

        let neighbors = point.get_neighbor_positions(x_bound, y_bound);

        assert_eq!(exp, neighbors);
    }
}
