use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Range {
    dest: u64,
    src: u64,
    range_length: u64,
}

impl Range {
    pub fn map(&self, value: &u64) -> Option<u64> {
        let r = self.src..(self.src + self.range_length);

        let contains = r.contains(value);
        if contains {
            let dest = value - self.src + self.dest;
            return Some(dest);
        }

        None
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<u64> = s
            .splitn(3, ' ')
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Range {
            dest: *parts.first().ok_or("Expected dest")?,
            src: *parts.get(1).ok_or("Expected src")?,
            range_length: *parts.get(2).ok_or("Expected range_length")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_parses() {
        let s = "11 22 33";
        let exp = Range { dest: 11, src: 22, range_length: 33 };

        let range: Range = s.parse().unwrap();

        assert_eq!(exp, range);
    }

    #[test]
    fn range_maps() {
        let exps = vec![
            (Some(5), "0 10 9".parse::<Range>().unwrap().map(&15)),
            (None,     "0 10 9".parse::<Range>().unwrap().map(&5)),
        ];

        for (exp, res) in exps {
            assert_eq!(exp, res);
        }
    }
}
