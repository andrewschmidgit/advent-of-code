use std::{collections::HashSet, env, fs};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let input = fs::read_to_string(filename).unwrap();
    let grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let (mut x, mut y, width) = get_guard_position(&input).unwrap();

    dbg!(x, y);

    let directions = [[0, -1], [1, 0], [0, 1], [-1, 0]];
    let mut direction_index = 0;
    let mut visited_positions = vec![(x, y)];

    loop {
        let [dx, dy] = directions[direction_index];
        (x, y) = (x + dx, y + dy);

        let traversed_x_bounds = x < 0 || x >= width as i32;
        let traversed_y_bounds = y < 0 || y >= input.lines().count() as i32;

        if traversed_x_bounds || traversed_y_bounds {
            break;
        }

        let c = grid[y as usize][x as usize];

        if c == '#' {
            (x, y) = (x - dx, y - dy);
            direction_index = (direction_index + 1) % directions.len();
            continue;
        }

        visited_positions.push((x, y));
    }

    dbg!(&visited_positions.len());
    dbg!(&visited_positions.iter().collect::<HashSet<_>>().len());
}

fn get_guard_position(input: &str) -> Result<(i32, i32, usize), String> {
    let width = input.lines().next().unwrap().len();
    let guard_pos = input
        .lines()
        .flat_map(|l| l.chars())
        .enumerate()
        .find_map(|(pos, c)| if c == '^' { Some(pos) } else { None })
        .ok_or("should have a guard position")?;

    let guard_x = guard_pos as i32 % width as i32;
    let guard_y = guard_pos as i32 / width as i32;
    Ok((guard_x, guard_y, width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_guard_position() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

        let (x, y, width) = get_guard_position(input).unwrap();

        assert_eq!(x, 4);
        assert_eq!(y, 6);
        assert_eq!(width, 10);
    }
}
