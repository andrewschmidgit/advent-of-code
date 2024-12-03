use map::Map;
use traversal::traverse;

mod map;
mod traversal;

pub fn run(contents: &str) -> Result<(), String>{
    let map: Map = contents.parse()?;
    let steps = human(&map)?;

    println!("solution 1 (ZZZ): {}", steps);

    let spooky_steps = spooky(&map)?;

    println!("solution 2 (**Z): {}", spooky_steps);

    Ok(())
}

fn human(map: &Map) -> Result<u64, String> {
    let s = traverse(map, "AAA")?;

    Ok(s)
}

fn spooky(map: &Map) -> Result<u64, String> {
    let key_steps = map.nodes
        .keys()
        .filter_map(|k| {
            if !k.ends_with('A') { return None; }
            Some(traverse(map, k))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let lcm = lcm_multi(key_steps)?;

    Ok(lcm)
}

fn lcm_multi(ns: Vec<u64>) -> Result<u64, String> {
    let mut last_lcm = *ns.first().ok_or("ns should not be empty")?;
    for n in ns.iter() {
        last_lcm = least_common_multiple(n, &last_lcm);
    }

    Ok(last_lcm)
}

fn least_common_multiple(a: &u64, b: &u64) -> u64 {
    let gcd = gcd(a, b);

    (a * b) / gcd
}

fn gcd(a: &u64, b: &u64) -> u64 {
    let mut a = *a;
    let mut b = *b;

    let mut r: u64;

    while (a % b) > 0 {
        r = a % b;
        a = b;
        b = r;
    }

    b
}
