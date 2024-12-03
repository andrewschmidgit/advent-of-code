use std::error::Error;

mod grid;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cosmos {
    Space,
    Galaxy,
}

#[derive(Debug, PartialEq, Eq)]
struct Universe {
    pub cosmos: Vec<Vec<Cosmos>>,
    pub galaxy_count: u32
}

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut universe = get_universe(contents);

    expand(&mut universe)?;

    todo!()
}

fn get_universe(s: &str) -> Universe {
    let mut galaxy_count = 0;
    let cosmos = s
        .lines()
        .map(|l| l.chars().map(|c| match c {
            '#' => {
                galaxy_count += 1;
                Cosmos::Galaxy
            },
            _ => Cosmos::Space,
        }).collect())
        .collect();

    Universe {
        cosmos,
        galaxy_count,
    }
}

fn expand(u: &mut Universe) -> Result<(), Box<dyn Error>> {
    let clone = u.cosmos.clone();

    let mut adjustments = 0;
    for (y, row) in clone.iter().enumerate() {
        let is_empty = row.iter().all(|c| c == &Cosmos::Space);
        if is_empty {
            u.cosmos.insert(y + adjustments, vec![Cosmos::Space; row.len()]);
            adjustments += 1;
        }
    }

    // Need to do this to get updated values
    let clone = u.cosmos.clone();
    let first_row = clone.get(0).ok_or("expected first line")?;
    let cols: Vec<Vec<_>> = first_row
        .iter()
        .enumerate()
        .map(|(x, _)| clone
            .iter()
            .map(|row| row[x])
            .collect())
        .collect();
        
    let mut adjustments = 0;
    for (x, col) in cols.iter().enumerate() {
        let is_empty = col.iter().all(|c| c == &Cosmos::Space);
        if is_empty {
            (0..clone.len()).for_each(|row_index| {
                u.cosmos[row_index].insert(x + adjustments, Cosmos::Space);
            });
            adjustments += 1;
        }
    }

    Ok(())
}

fn format_image(a: &[Vec<Cosmos>]) -> String {
    let mut s = String::new();
    for row in a.iter() {
        for c in row.iter() {
            let c = match c {
                Cosmos::Space => '.',
                Cosmos::Galaxy => '#',
            };
            s.push(c);
        }

        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fix_simple() {
        let mut u = get_universe(r"..
.#");
        let exp = get_universe(r"...
...
..#");

        // println!("Before");
        // println!("{}", format_image(&array));

        expand(&mut u).unwrap();

        // println!("\nAfter");
        // println!("{}", format_image(&array));

        assert_eq!(u, exp);
    }

    #[test]
    fn fix_sample() {
        let mut universe = get_universe(r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");

        let exp = get_universe(r"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......");

        // println!("Before");
        // println!("{}", format_image(&array));

        expand(&mut universe).unwrap();

        // println!("\nAfter");
        // println!("{}", format_image(&array));

        assert_eq!(universe, exp);
    }
}
