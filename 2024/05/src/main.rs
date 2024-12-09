use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = args.get(1).expect("should provide filename");
    let contents = fs::read_to_string(filename).unwrap();

    print::run(&contents).unwrap();
}
