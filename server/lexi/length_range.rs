use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LengthRange {
    min: usize,
    max: usize,
}

impl LengthRange {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, n: usize) -> bool {
        (self.min..=self.max).contains(&n)
    }
}

impl FromStr for LengthRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn try_parse(s: &str) -> Option<LengthRange> {
            let mut parts = s.split('-');
            let min = parts.next()?.parse().ok()?;
            let max = match parts.next() {
                Some(max) => max.parse().ok()?,
                None => min,
            };
            Some(LengthRange::new(min, max))
        }
        try_parse(s).ok_or_else(|| format!("Invalid length range: {}", s))
    }
}

impl TryFrom<&str> for LengthRange {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl<T> TryFrom<Option<T>> for LengthRange
where
    LengthRange: TryFrom<T>,
{
    type Error = String;
    fn try_from(s: Option<T>) -> Result<Self, Self::Error> {
        if let Some(s) = s {
            s.try_into().map_err(|_| "Invalid length range".to_owned())
        } else {
            Err("Empty length range".to_owned())
        }
    }
}

impl Display for LengthRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.min, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_parse() {
        let range: LengthRange = "4-5".parse().unwrap();
        check!(range.min == 4);
        check!(range.max == 5);
    }

    #[test]
    fn test_single_number() {
        let range: LengthRange = "4".parse().unwrap();
        check!(range.min == 4);
        check!(range.max == 4);
    }
}
