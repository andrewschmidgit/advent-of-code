use std::error::Error;

use report::Report;
mod report;

pub fn run(contents: &str) -> Result<(), Box<dyn Error>> {
    let reports = parse(contents)?;

    let safe_reports = reports.iter().filter(|r| r.is_safe()).count();

    println!("safe reports: {}", safe_reports);

    Ok(())
}

fn parse(contents: &str) -> Result<Vec<Report>, Box<dyn Error>> {
    Ok(contents
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Report>, _>>()?)
}

#[test]
fn parse_succeeds() {}
