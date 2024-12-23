use tree::part2;

mod tree;

fn main() {
    let count = part2("572556 22 0 528 4679021 1 10725 2790", 25);
    println!("pt1 stones: {}", count);

    let count = part2("572556 22 0 528 4679021 1 10725 2790", 75);
    println!("pt2 stones: {}", count);
}
