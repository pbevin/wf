use super::LengthRange;
use super::LetterMask;
use super::SortedLetters;
use super::Entry;

#[derive(Debug, Clone)]
pub enum Check {
    SingleWord(bool),
    Length(LengthRange),
    ExcludeLetters(LetterMask),
    IncludeLetters(LetterMask),
    Contains(SortedLetters),
    Contained(SortedLetters),
}

#[derive(Debug, Clone, Default)]
pub struct Filter {
    checks: Vec<Check>,
}

impl Filter {
    pub fn new(checks: Vec<Check>) -> Self {
        Self { checks }
    }

    pub fn is_empty(&self) -> bool {
        self.checks.is_empty()
    }

    pub fn matches(&self, entry: &Entry) -> bool {
        for check in &self.checks {
            match check {
                Check::SingleWord(want_single) => {
                    if entry.is_single_word() != *want_single {
                        return false;
                    }
                }

                Check::Length(range) => {
                    if !range.contains(entry.num_letters()) {
                        return false;
                    }
                }
                Check::ExcludeLetters(mask) => {
                    if mask.0 & entry.mask.0 != 0 {
                        return false;
                    }
                }
                Check::IncludeLetters(mask) => {
                    if mask.0 & entry.mask.0 != mask.0 {
                        return false;
                    }
                }
                Check::Contains(sorted) => {
                    if !entry.sorted.is_superset(sorted) {
                        return false;
                    }
                }
                Check::Contained(sorted) => {
                    if !entry.sorted.is_subset(sorted) {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[derive(Default)]
pub struct FilterBuilder {
    checks: Vec<Check>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single_word(mut self, want_single: Option<bool>) -> Self {
        if let Some(want_single) = want_single {
            self.checks.push(Check::SingleWord(want_single));
        }
        self
    }

    pub fn length(mut self, range: impl TryInto<LengthRange>) -> Self {
        if let Ok(range) = range.try_into() {
            self.checks.push(Check::Length(range));
        }
        self
    }

    pub fn exclude_letters(mut self, letters: impl TryInto<LetterMask>) -> Self {
        if let Ok(mask) = letters.try_into() {
            self.checks.push(Check::ExcludeLetters(mask));
        }
        self
    }

    pub fn include_letters(mut self, letters: impl TryInto<LetterMask>) -> Self {
        if let Ok(mask) = letters.try_into() {
            self.checks.push(Check::IncludeLetters(mask));
        }
        self
    }

    pub fn contains(mut self, letters: impl TryInto<SortedLetters>) -> Self {
        if let Ok(sorted) = letters.try_into() {
            self.checks.push(Check::Contains(sorted));
        }
        self
    }

    pub fn contained(mut self, letters: impl TryInto<SortedLetters>) -> Self {
        if let Ok(sorted) = letters.try_into() {
            self.checks.push(Check::Contained(sorted));
        }
        self
    }

    pub fn build(self) -> Filter {
        Filter::new(self.checks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexi::Lexicon;

    #[test]
    fn test_include_letters() {
        let filter = Filter::new(vec![Check::IncludeLetters(LetterMask::new("abc"))]);
        let lex = Lexicon::from_iter(["back", "cat", "taxicab"].into_iter());

        assert_eq!(
            lex.matching_words(&filter).collect::<Vec<_>>(),
            ["back", "taxicab"]
        );
    }

    #[test]
    fn test_exclude_letters() {
        let filter = Filter::new(vec![Check::ExcludeLetters(LetterMask::new("abc"))]);
        let lex = Lexicon::from_iter(["debauched", "squirming", "zeitgeist"].into_iter());

        assert_eq!(
            lex.matching_words(&filter).collect::<Vec<_>>(),
            ["squirming", "zeitgeist"]
        );
    }

    #[test]
    fn test_length() {
        let filter = Filter::new(vec![Check::Length(LengthRange::new(4, 6))]);
        let lex = Lexicon::from_iter(["back", "cat", "plinth", "taxicab"].into_iter());

        assert_eq!(
            lex.matching_words(&filter).collect::<Vec<_>>(),
            ["back", "plinth"]
        );
    }

    #[test]
    fn test_contains() {
        let filter = Filter::new(vec![Check::Contains(SortedLetters::from_word("lmn"))]);

        let lex = Lexicon::from_iter(
            [
                "equanimity",
                "repose",
                "calmness",
                "coolness",
                "placidity",
                "serenity",
                "composure",
                "tranquility",
            ]
            .into_iter(),
        );

        assert_eq!(
            lex.matching_words(&filter).collect::<Vec<_>>(),
            ["calmness"]
        );
    }
}
