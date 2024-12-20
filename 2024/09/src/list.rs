use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Block {
    Free,
    File(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blocks {
    list: Vec<(Block, u32)>,
}

impl Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (block, count) in self.list.iter() {
            let char = match block {
                Block::Free => ".".to_string(),
                Block::File(id) => id.to_string(),
            };

            let s = char.repeat(*count as usize);

            write!(f, "{}", s)?;
        }

        Ok(())
    }
}

impl Blocks {
    pub fn new(list: Vec<(Block, u32)>) -> Self {
        Self { list }
    }

    pub fn checksum(&self) -> u64 {
        let mut index = 0;
        let mut total = 0;
        for (block, size) in self.list.iter() {
            if let Block::File(id) = block {
                let sum: u64 = (index..index + size).map(|i| i as u64 * *id as u64).sum();
                total += sum;
            }
            index = index + size;
        }

        total
    }

    pub fn compress(&mut self) {
        for i in (0..self.list.len()).rev() {
            let (file_block, file_size) = self.list[i];

            let Block::File(_) = file_block else {
                continue;
            };

            for j in 0..i {
                let (free_block, free_size) = self.list[j];
                let is_free = free_block == Block::Free;
                let free_can_fit_file = free_size >= file_size;
                if !is_free || !free_can_fit_file {
                    continue;
                }

                if let Some(free) = self.list.get_mut(j) {
                    free.1 -= file_size;
                }

                let file = self.list.remove(i);
                self.list.insert(j, file);
                self.list.insert(i, (free_block, file_size));

                break;
            }
        }
    }
}

pub fn parse_list(s: &str) -> Blocks {
    let mut list = vec![];
    for (i, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if i % 2 == 0 {
                // file
                list.push((Block::File(i / 2), digit));
            } else {
                list.push((Block::Free, digit));
            }
        }
    }

    Blocks::new(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_parses() {
        let list = parse_list("12345");

        assert_eq!(list.to_string(), "0..111....22222");
    }

    #[test]
    fn list_checksums() {
        let list = Blocks {
            list: vec![
                (Block::File(0), 2),
                (Block::File(9), 2),
                (Block::File(8), 1),
                (Block::File(1), 3),
                (Block::File(8), 3),
                (Block::File(2), 1),
                (Block::File(7), 3),
                (Block::File(3), 3),
                (Block::File(6), 1),
                (Block::File(4), 2),
                (Block::File(6), 1),
                (Block::File(5), 4),
                (Block::File(6), 2),
                (Block::Free, 14),
            ],
        };
        assert_eq!(list.checksum(), 1928);
    }

    #[test]
    fn list_compresses() {
        let mut list = parse_list("2333133121414131402");

        assert_eq!(
            list.to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );

        list.compress();

        assert_eq!(
            list.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );

        assert_eq!(list.checksum(), 2858);
    }
}
