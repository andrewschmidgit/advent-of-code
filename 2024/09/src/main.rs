mod list;

use std::{
    collections::BTreeMap,
    env,
    fmt::{self, Display},
    fs,
    ops::Range,
};

use list::parse_list;

type Blocks = Vec<Block>;
type Span = Range<u32>;
type Files = BTreeMap<u32, Span>;
type Free = Vec<Span>;

fn main() {
    let filename = env::args().skip(1).next().expect("filename");
    let input = fs::read_to_string(&filename).expect("file");

    let mut blocks = parse(&input);
    compress(&mut blocks);
    let cs = checksum(&blocks);
    println!("pt1: checksum: {}", cs);

    let mut blocks = parse_list(&input);
    blocks.compress();
    println!("pt2: checksum: {}", blocks.checksum());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    Free,
    File(u32),
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Block::Free => ".",
            Block::File(id) => &id.to_string(),
        };
        write!(f, "{c}")
    }
}

fn parse(s: &str) -> Blocks {
    let mut blocks = vec![];
    for (i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            let block_type = if i % 2 == 0 {
                // file
                Block::File(i as u32 / 2)
            } else {
                Block::Free
            };

            let mut new_blocks = vec![block_type; d as usize];
            blocks.append(&mut new_blocks);
        }
    }

    blocks
}

fn display(blocks: &Blocks) -> String {
    blocks.iter().map(|b| b.to_string()).collect()
}

fn compress(blocks: &mut Blocks) {
    let mut front_i = 0;
    let mut back_i = blocks.len() - 1;

    while front_i < back_i {
        let front = blocks[front_i];
        let back = blocks[back_i];

        if front != Block::Free {
            front_i += 1;
            continue;
        }

        if back == Block::Free {
            back_i -= 1;
            continue;
        }

        blocks[front_i] = back;
        blocks[back_i] = front;

        front_i += 1;
        back_i -= 1;
    }
}

fn checksum(blocks: &Blocks) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| {
            if let Block::File(id) = b {
                Some(*id as usize * i)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let blocks = parse("12345");
        assert_eq!(display(&blocks), "0..111....22222");

        let blocks = parse("2333133121414131402");
        assert_eq!(
            display(&blocks),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn compresses() {
        let mut blocks = parse("12345");
        compress(&mut blocks);
        assert_eq!(display(&blocks), "022111222......");

        let mut blocks = parse("2333133121414131402");
        compress(&mut blocks);
        assert_eq!(
            display(&blocks),
            "0099811188827773336446555566.............."
        );
    }

    #[test]
    fn checksums() {
        let mut blocks = parse("2333133121414131402");
        compress(&mut blocks);
        let cs = checksum(&blocks);

        assert_eq!(cs, 1928)
    }
}
