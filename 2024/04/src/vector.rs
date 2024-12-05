use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(p: Point, q: Point) -> Self {
        Self {
            x: q.x as i32 - p.x as i32,
            y: q.y as i32 - p.y as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_initializes() {
        let tests = vec![
            (
                Point { x: 0, y: 0 },
                Point { x: 1, y: 1 },
                Vector { x: 1, y: 1 },
            ),
            (
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 },
                Vector { x: -1, y: -1 },
            ),
            (
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Vector { x: 1, y: -1 },
            ),
            (
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Vector { x: -1, y: 1 },
            ),
        ];

        for (p, q, e) in tests {
            let v = Vector::new(p, q);
            assert_eq!(v, e);
        }
    }
}
