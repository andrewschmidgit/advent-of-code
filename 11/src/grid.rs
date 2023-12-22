#[derive(Debug)]
pub struct Grid<T> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where 
    T: Default + Clone
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            array: vec![T::default(); width * height],
            width,
            height,
        }
    }
}
