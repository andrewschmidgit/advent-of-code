use std::{cmp::Ordering, error::Error, str::FromStr};

mod rule;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let (rules, updates) = contents
        .split_once("\n\n")
        .ok_or("expected double newline")?;

    let rules = rules
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Rule>, _>>()?;

    let mut updates: Vec<Vec<usize>> = updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse())
                .collect::<Result<Vec<usize>, _>>()
        })
        .collect::<Result<Vec<Vec<usize>>, _>>()?;

    let mut ordered_middle_sum = 0;

    for update in updates.iter_mut() {
        println!("checking update: {:?}", update);

        let mut violations: Vec<Violation> = vec![];
        for rule in rules.iter() {
            if let Some(violation) = rule.evaluate(&update) {
                violations.push(violation);
            }
        }

        if violations.len() == 0 {
            ordered_middle_sum += update[(update.len() - 1) / 2];
        }
    }

    let order = |a: &usize, b: &usize| -> Ordering {
        if rules.contains(&Rule {
            left: *a,
            right: *b,
        }) {
            Ordering::Less
        } else if rules.contains(&Rule {
            left: *b,
            right: *a,
        }) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut sum = 0;
    for update in updates.iter_mut() {
        if !update.is_sorted_by(|a, b| order(a, b) != Ordering::Greater) {
            update.sort_by(order);
            sum += update[update.len() / 2];
        }
    }

    println!("ordered sum: {}", ordered_middle_sum);
    println!("sum: {}", sum);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    left: usize,
    right: usize,
}

impl Rule {
    pub fn evaluate(&self, list: &[usize]) -> Option<Violation> {
        let mut right_index: Option<usize> = None;

        for (i, el) in list.iter().enumerate() {
            if let Some(right_index) = right_index {
                if *el == self.left {
                    return Some(Violation {
                        left_index: i,
                        right_index,
                    });
                }
            } else if *el == self.left {
                return None;
            } else if *el == self.right {
                right_index = Some(i);
            }
        }

        None
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("|").ok_or("could not fine a '|'")?;

        Ok(Self {
            left: left.parse()?,
            right: right.parse()?,
        })
    }
}

#[derive(Debug)]
struct Violation {
    left_index: usize,
    right_index: usize,
}
