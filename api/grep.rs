use crate::lexi::Lexicon;
use regex::RegexBuilder;

pub fn search(spec: &super::GrepSpec) {
    let re = RegexBuilder::new(&spec.regex).case_insensitive(spec.case_insensitive).build().unwrap();
    let lexi = Lexicon::load();
    lexi.entries().filter(|entry| re.is_match(entry.word())).for_each(|entry| {
        println!("{}", entry.word());
    });
}
