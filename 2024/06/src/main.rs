use std::{collections::HashSet, env, fs};

type Point = (i32, i32);
type Map = Vec<Vec<char>>;

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let grid = parse(&input);

    let guard = get_guard_position(&grid).expect("should find a guard");

    // Part 1
    let path = sim(&grid, &guard).expect("should be a valid path");
    let unique_positions = path.into_iter().collect::<HashSet<_>>();
    println!("unique visited positions: {}", unique_positions.len());

    // Part 2
    let mut infinite_loop_positions = vec![];
    let mut grid = grid;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '#' || grid[y][x] == '^' {
                continue;
            }

            grid[y][x] = '#';
            if sim(&grid, &guard).is_err() {
                infinite_loop_positions.push((x, y));
            }
            grid[y][x] = '.';
        }
    }

    println!(
        "possible obstacle positions: {}",
        infinite_loop_positions.len()
    );
}

fn parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn get_guard_position(map: &Map) -> Option<Point> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .find(|(_, _, c)| **c == '^')
        .map(|(x, y, _)| (x as i32, y as i32))
}

#[derive(Debug, PartialEq, Eq)]
enum SimError {
    InfiniteLoop,
}
fn sim(map: &Map, start: &Point) -> Result<Vec<Point>, SimError> {
    let (mut x, mut y) = start;
    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let directions = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    let mut direction_index = 0;
    let mut path = vec![(x, y)];

    let mut visited_vector = HashSet::new();

    loop {
        let [dx, dy] = directions[direction_index];
        (x, y) = (x + dx, y + dy);

        let traversed_x_bounds = x < 0 || x >= width;
        let traversed_y_bounds = y < 0 || y >= height;

        if traversed_x_bounds || traversed_y_bounds {
            break;
        }

        let c = map[y as usize][x as usize];

        if c == '#' {
            (x, y) = (x - dx, y - dy);
            direction_index = (direction_index + 1) % directions.len();
            continue;
        }

        if !visited_vector.insert((x, y, dx, dy)) {
            return Err(SimError::InfiniteLoop);
        }

        path.push((x, y));
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_guard_position() {
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
",
        );

        let (x, y) = get_guard_position(&map).unwrap();

        assert_eq!(x, 4);
        assert_eq!(y, 6);
    }

    #[test]
    fn sims() {
        let map = parse(
            ".#..
...#
.^..
....",
        );

        let expected = vec![(1, 2), (1, 1), (2, 1), (2, 2), (2, 3)];

        let path = sim(&map, &(1, 2)).unwrap();

        assert_eq!(path, expected);

        let map = parse(
            ".#..
...#
#...
.^#.",
        );

        let err = sim(&map, &(1, 2)).unwrap_err();

        assert_eq!(err, SimError::InfiniteLoop)
    }

    #[test]
    fn sim_infinite_loops() {
        // Option 1
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#...",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);

        // Option 2
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
......#.#.
#.........
......#...",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);

        // Option 3
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
.......##.
#.........
......#...",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);

        // Option 4
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
##........
......#...",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);

        // Option 5
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#..#......
......#...",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);

        // Option 6
        let map = parse(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......##..
",
        );

        let guard = get_guard_position(&map).unwrap();
        let err = sim(&map, &guard).unwrap_err();
        assert_eq!(err, SimError::InfiniteLoop);
    }
}
