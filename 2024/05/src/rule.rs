use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    Pending(usize, usize),
    Tripped(usize),
    Passed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn is_valid(&self) -> bool {
        self.rules.iter().all(|r| !matches!(r, Rule::Failed))
    }

    pub fn evaluate(&mut self, value: usize) -> bool {
        for rule in self.rules.iter_mut() {
            match rule {
                Rule::Pending(left, right) => {
                    if *left == value {
                        *rule = Rule::Passed;
                    } else if *right == value {
                        *rule = Rule::Tripped(*left);
                    }
                }
                Rule::Tripped(left) => {
                    if *left == value {
                        *rule = Rule::Failed;
                        return true;
                    }
                }
                Rule::Passed => {}
                Rule::Failed => {}
            }
        }

        return false;
    }
}

impl FromStr for RuleSet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(|l| -> Result<Rule, Box<dyn Error>> {
                let (left, right) = l.split_once("|").ok_or("could not fine a '|'")?;
                Ok(Rule::Pending(left.parse()?, right.parse()?))
            })
            .collect::<Result<Vec<Rule>, _>>()?;

        Ok(Self { rules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ruleset_parses() {
        let fail = "1234".parse::<RuleSet>();
        assert!(fail.is_err());

        let fail = "ab|cd".parse::<RuleSet>();
        assert!(fail.is_err());

        let rules: RuleSet = "12|34".parse().unwrap();
        let rule = rules.rules.get(0).unwrap();
        assert_eq!(rule, &Rule::Pending(12, 34));

        let rules: RuleSet = "12|34\n56|78".parse().unwrap();

        let rule = rules.rules.get(0).unwrap();
        assert_eq!(rule, &Rule::Pending(12, 34));
        let rule = rules.rules.get(1).unwrap();
        assert_eq!(rule, &Rule::Pending(56, 78));
    }

    #[test]
    fn ruleset_evaluation_passes() {
        let mut rules: RuleSet = "12|34\n56|78".parse().unwrap();

        rules.evaluate(12);
        let rule = rules.rules.get(0).unwrap();
        assert_eq!(rule, &Rule::Passed);

        rules.evaluate(56);
        rules.evaluate(78);
        let rule = rules.rules.get(1).unwrap();
        assert_eq!(rule, &Rule::Passed);
    }

    #[test]
    fn ruleset_evaluation_trips() {
        let mut rules: RuleSet = "12|34\n56|78".parse().unwrap();

        rules.evaluate(34);
        let rule = rules.rules.get(0).unwrap();
        assert_eq!(rule, &Rule::Tripped(12));
    }

    #[test]
    fn ruleset_evaluation_fails() {
        let mut rules: RuleSet = "12|34\n56|78".parse().unwrap();

        rules.evaluate(34);
        rules.evaluate(12);
        let rule = rules.rules.get(0).unwrap();
        assert_eq!(rule, &Rule::Failed);
    }

    #[test]
    fn ruleset_validity() {
        let mut rules: RuleSet = "12|34\n56|78".parse().unwrap();
        assert!(rules.is_valid());

        rules.evaluate(12);
        assert!(rules.is_valid());

        rules.evaluate(34);
        assert!(rules.is_valid());

        rules.evaluate(78);
        assert!(rules.is_valid());

        let tripped = rules.evaluate(56);
        assert!(tripped);
    }
}
