use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, color) = s.split_once(' ').ok_or("Color: No space in string")?;

        let value_fromstr = value.parse::<u32>().map_err(|_| "Could not parse value")?;

        let color = match color {
            "red" => Ok(Color::Red(value_fromstr)),
            "green" => Ok(Color::Green(value_fromstr)),
            "blue" => Ok(Color::Blue(value_fromstr)),
            _ => Err("Invalid color")
        }?;

        Ok(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_parses() {
        let expectations = vec![
            ("4 red", Color::Red(4)),
            ("12 green", Color::Green(12)),
            ("3 blue", Color::Blue(3)),
        ];

        for (str, ex) in expectations {
            assert_eq!(str.parse::<Color>().unwrap(), ex);
        }
    }
}
