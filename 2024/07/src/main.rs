use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::{env, fs};

fn main() {
    let filename = env::args().skip(1).next().unwrap();
    let input = fs::read_to_string(&filename).unwrap();

    let es = equations(&input);

    // part 1
    let result = get_total_calibration(&es, &[Operator::Add, Operator::Multiply]);
    println!("total_calibration_result: {}", result);

    // part 2
    let result = get_total_calibration(&es, &[Operator::Add, Operator::Multiply, Operator::Concat]);
    println!("total_calibration_result: {}", result);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug, PartialEq, Eq)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    pub fn find_operator_configurations<'a>(&self, ops: &'a [Operator]) -> Vec<Vec<&'a Operator>> {
        let num_operators = self.operands.len() - 1;

        let operator_variations = generate_operators(num_operators as u32, ops);

        let mut working_operator_variations = vec![];

        for v in operator_variations {
            let mut v_iter = v.iter();

            let result = self
                .operands
                .clone()
                .into_iter()
                .reduce(|a, b| match v_iter.next().unwrap() {
                    Operator::Add => a + b,
                    Operator::Multiply => a * b,
                    Operator::Concat => {
                        let concat: String = a.to_string() + &b.to_string();
                        u64::from_str_radix(&concat, 10).unwrap()
                    }
                })
                .unwrap();

            if result == self.result {
                working_operator_variations.push(v);
            }
        }

        working_operator_variations
    }
}

fn equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|l| equation(l).map(|(_, e)| e).ok())
        .collect()
}

fn equation<'a>(input: &'a str) -> IResult<&'a str, Equation> {
    let number_parser =
        |number_str: &'a str| map_res(digit1, |s: &'a str| u64::from_str_radix(s, 10))(number_str);
    let (input, result) = terminated(number_parser, tag(": "))(input)?;
    let (input, operands) = separated_list1(space1, number_parser)(input)?;

    Ok((input, Equation { result, operands }))
}

fn generate_operators(length: u32, supported_operators: &[Operator]) -> Vec<Vec<&Operator>> {
    let radix = supported_operators.len();
    let upper_bound = supported_operators.len().pow(length);

    (0..upper_bound)
        .map(|n| {
            (0..length)
                .filter_map(|d| {
                    let index = (n / radix.pow(d)) % radix;

                    supported_operators.get(index)
                })
                .collect()
        })
        .collect()
}

fn get_total_calibration(es: &Vec<Equation>, ops: &[Operator]) -> u64 {
    es.iter()
        .filter_map(|e| {
            if e.find_operator_configurations(ops).len() > 0 {
                Some(e.result)
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
    fn generates_operators() {
        let ops = generate_operators(2, &[Operator::Add, Operator::Multiply]);
        let exp = vec![
            vec![&Operator::Add, &Operator::Add],
            vec![&Operator::Multiply, &Operator::Add],
            vec![&Operator::Add, &Operator::Multiply],
            vec![&Operator::Multiply, &Operator::Multiply],
        ];

        assert_eq!(ops, exp);
    }

    #[test]
    fn equation_gets_parsed() {
        let expectations = [
            (
                "190: 10 19",
                Equation {
                    result: 190,
                    operands: vec![10, 19],
                },
            ),
            (
                "3267: 81 40 27",
                Equation {
                    result: 3267,
                    operands: vec![81, 40, 27],
                },
            ),
        ];

        for (s, exp) in expectations {
            let (_, equation) = equation(s).unwrap();
            assert_eq!(equation, exp);
        }
    }

    #[test]
    fn gets_configurations() {
        let es = equations(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        for e in es {
            let configs = e.find_operator_configurations(&[Operator::Add, Operator::Multiply]);
            println!("{:?}: {}", e, configs.len());
        }
    }
}
