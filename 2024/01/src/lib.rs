use std::{
    collections::{HashMap, HashSet},
    error::Error,
    iter::zip,
};

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let (mut list0, mut list1) = parse(contents)?;

    list0.sort();
    list1.sort();

    let distance: i32 = zip(list0.iter(), list1.iter())
        .map(|(num0, num1)| (num0 - num1).abs())
        .sum();

    println!("distance: {}", distance);

    let freq0 = freq_map(list0);
    let freq1 = freq_map(list1);

    let mut similarity = 0;
    for (num, freq) in freq0.iter() {
        if let Some(other_freq) = freq1.get(num) {
            similarity += *num as usize * freq * other_freq;
        }
    }

    println!("similarity: {}", similarity);

    Ok(())
}

fn parse(contents: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut list0: Vec<i32> = vec![];
    let mut list1: Vec<i32> = vec![];

    for line in contents.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();

        if numbers.len() != 2 {
            println!("Could not parse line: '{}', skipping", line);
            continue;
        }

        list0.push(numbers[0].parse()?);
        list1.push(numbers[1].parse()?);
    }

    Ok((list0, list1))
}

fn freq_map(list: Vec<i32>) -> HashMap<i32, usize> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for el in list {
        *m.entry(el).or_default() += 1;
    }

    return m;
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn succeeds() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let result = parse(input);
        let (list1, list2) = result.expect("to succeed");

        assert_eq!(list1, [3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, [4, 3, 5, 3, 9, 3]);
    }
}

#[test]
fn freq_map_succeeds() {
    let list = vec![4, 1, 4, 2, 2, 4, 3, 3, 3, 4];

    let map = freq_map(list);

    let mut expected = HashMap::new();
    expected.insert(1, 1);
    expected.insert(2, 2);
    expected.insert(3, 3);
    expected.insert(4, 4);
    assert_eq!(map, expected)
}
