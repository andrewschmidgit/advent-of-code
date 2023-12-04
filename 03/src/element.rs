#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Element {
    None,
    Number(u32),
    Symbol(char),
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        if value == '.' {
            return Self::None;
        }

        if let Some(d) = value.to_digit(10) {
            return Self::Number(d);
        }

        Self::Symbol(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn element_parses_none() {
        let c = '.';
        let exp = Element::None;

        assert_eq!(exp, c.into());
    }

    #[test]
    fn element_parses_number() {
        let c = '7';
        let exp = Element::Number(7);

        assert_eq!(exp, c.into());
    }

    #[test]
    fn element_parses_symbol() {
        let c = '$';
        let exp = Element::Symbol(c);

        assert_eq!(exp, c.into());
    }
}
