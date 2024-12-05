use crate::vector::Vector;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn transform(self: &Self, v: &Vector) -> Result<Self, &'static str> {
        let x = self.x as i32 + v.x;
        let y = self.y as i32 + v.y;

        if x < 0 || y < 0 {
            return Err("");
        }

        Ok(Self {
            x: x as usize,
            y: y as usize,
        })
    }
}
