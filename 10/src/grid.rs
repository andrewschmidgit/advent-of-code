use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from_p(p: Point, x_t: i32, y_t: i32) -> Self {
        Self {
            x: (p.x as i32 + x_t) as usize,
            y: (p.y as i32 + y_t) as usize,
        }
    }

    pub fn distance_from(&self, other: &Point) -> (i32, i32) {
        (
            self.x as i32 - other.x as i32,
            self.y as i32 - other.y as i32,
        )
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    array: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Copy
    {
        Self {
            array: [T::default()].repeat(width * height),
            width,
            height,
        }
    }

    pub fn get(&self, p: Point) -> &T {
        &self.array[p.y * self.width + p.x]
    }

    pub fn get_mut(&mut self, p: Point) -> &mut T {
        self.array.get_mut(p.y * self.width + p.x).unwrap()
    }

    pub fn find(&self, pipe: &T) -> Option<Point> 
    where
        T: PartialEq
    {
        let (i, _) = self.array
            .iter()
            .enumerate()
            .find(|(_, p)| *p == pipe)?;

        Some(Point {
            x: i % self.width,
            y: i / self.height,
        })
    }

    pub fn set_all<F>(&mut self, set: F)
    where
        F: Fn(Point) -> T {
        self.array
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| {
                *v = set(Point {
                    x: i % self.width,
                    y: i / self.width,
                });
            })
    }
}
