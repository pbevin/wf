use crate::lexi::Entry;
use crate::lexi::FilterBuilder;
use crate::lexi::Lexicon;
use crate::lexi::SortedLetters;
use itertools::Itertools;
use std::cmp::Reverse;

pub struct Anagrams<'a> {
    entries: Vec<&'a Entry<'a>>,
    stack: Vec<AnagramFrame<'a>>,
}

#[derive(Debug)]
pub struct AnagramFrame<'a> {
    letters: SortedLetters,
    pos: usize,
    partial_result: Vec<&'a Entry<'a>>,
    is_virgin: bool,
}

pub fn anagrams<'a>(
    term: &str,
    lexi: &'a Lexicon<'a>,
) -> impl Iterator<Item = (Vec<&'a Entry<'a>>, SortedLetters)> {
    let letters = SortedLetters::from_word(term);
    let filter = FilterBuilder::new()
        .contained(letters)
        .single_word(true.into())
        .build();
    let mut words = lexi
        .entries()
        .filter(|entry| filter.matches(entry))
        .collect_vec();
    words.sort_by_key(|entry| {
        (
            Reverse(lexi.rate(entry) as usize),
            Reverse(entry.word().len()),
            entry.word(),
        )
    });

    let frame = AnagramFrame {
        letters,
        pos: 0,
        partial_result: Vec::new(),
        is_virgin: true,
    };

    Anagrams {
        entries: words,
        stack: vec![frame],
    }
}

impl<'a> Iterator for Anagrams<'a> {
    type Item = (Vec<&'a Entry<'a>>, SortedLetters);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(frame) = self.stack.last_mut() {
            if frame.letters.is_empty() {
                let frame = self.stack.pop().unwrap();
                return Some((frame.partial_result, frame.letters));
            }
            if let Some(entry) = self.entries.get(frame.pos) {
                frame.pos += 1;

                if let Some(letters) = frame.letters.minus(entry.letters()) {
                    let mut result = frame.partial_result.clone();
                    result.push(entry);
                    let new_frame = AnagramFrame {
                        letters,
                        // Start the child frame at the next entry, ensuring
                        // that we eliminate duplicates like
                        //  MEATS = ME + AT + "s"
                        //  MEATS = AT + ME + "s"
                        pos: frame.pos,
                        partial_result: result,
                        is_virgin: true,
                    };
                    frame.is_virgin = false;
                    self.stack.push(new_frame);
                }
            } else {
                let frame = self.stack.pop().unwrap();
                if frame.is_virgin {
                    return Some((frame.partial_result, frame.letters));
                }
            }
        }
        None
    }
}
