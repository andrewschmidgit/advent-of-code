use grid::Grid;
use pipe::Pipe;
use walk::Walker;

mod grid;
mod pipe;
mod walk;

pub fn run(s: &str) {
    let grid = build_grid(s);

    let walker = Walker::new(grid).unwrap();

    let (d, _p) = walker.find_furthest_point().unwrap();

    println!("solution 1 (furthest): {}", d);

    let i = walker.get_enclosed_tiles();
    println!("solution 2 (enclosed tiles): {}", i);
}

fn build_grid(s: &str) -> Grid<Pipe> {
    let input: Vec<Vec<_>> = s
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let width = input[0].len();
    let height = input.len();

    let mut grid = Grid::new(width, height);
    grid.set_all(|p| {
        let c = input[p.y][p.x];
        Pipe::from_char(c)
    });

    grid
}

#[cfg(test)]
mod tests {
    use crate::grid::Point;

    use super::*;

    #[test]
    fn grid_builds() {
        let s = "S.|-LJ7F";

        let grid = build_grid(s);

        assert_eq!(&Pipe::Start, grid.get(Point::new(0, 0)));
        assert_eq!(&Pipe::Ground, grid.get(Point::new(1, 0)));

        assert_eq!(&Pipe::Pipe, grid.get(Point::new(2, 0)));
        assert_eq!(&Pipe::Dash, grid.get(Point::new(3, 0)));

        assert_eq!(&Pipe::NtoE, grid.get(Point::new(4, 0)));
        assert_eq!(&Pipe::WtoN, grid.get(Point::new(5, 0)));
        assert_eq!(&Pipe::WtoS, grid.get(Point::new(6, 0)));
        assert_eq!(&Pipe::EtoS, grid.get(Point::new(7, 0)));
    }
}
