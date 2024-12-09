use std::error::Error;

use rule::RuleSet;

mod rule;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let (rules, updates) = contents
        .split_once("\n\n")
        .ok_or("expected double newline")?;

    let ruleset: RuleSet = rules.parse()?;
    let updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse())
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<Vec<usize>>, _>>()?;

    let mut middle_page_sum = 0;

    for update in updates {
        let mut ruleset = ruleset.clone();
        let mut is_update_ordered = true;

        for n in update.iter() {
            let tripped = ruleset.evaluate(*n);

            if tripped {
                println!("{} violated a rule", n);
                is_update_ordered = false;
                break;
            }
        }

        if is_update_ordered {
            let middle_index = (update.len() - 1) / 2;
            middle_page_sum += update.get(middle_index).unwrap();

            println!("update {:?} is correctly ordered", update);
        }
    }

    println!("middle page number sum: {}", middle_page_sum);

    Ok(())
}
