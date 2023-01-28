#![warn(clippy::str_to_string)]
#![warn(clippy::if_then_some_else_none)]

mod anagrams;
mod assets;
mod grep;
mod lexi;
mod server;

pub use self::anagrams::anagrams;
use self::lexi::FilterBuilder;
use self::lexi::Lexicon;
use self::lexi::Popularity;
use self::lexi::{Filter, LengthRange, SortedLetters};
use clap::ArgGroup;
use clap::CommandFactory;
use clap::Parser;
use lexi::solve_anagram;
use owo_colors::OwoColorize;
use std::io::stdout;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

fn main() {
    install_tracing();
    let cmdline = Cmdline::parse();
    match cmdline.command {
        Subcommand::Server(opts) => server::start_sync(&opts),
        Subcommand::Search(filter) => search(filter),
        Subcommand::Grep(spec) => grep::search(&spec),
        Subcommand::Completions => gen_completions(),
    }
}

pub fn install_tracing() {
    let fmt = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_thread_names(true);

    let filter = EnvFilter::from_default_env();

    tracing_subscriber::registry().with(fmt).with(filter).init();
}

fn search(spec: FilterSpec) {
    let filter = spec.to_filter();
    if filter.is_empty() {
        println!("{}", "No filter specified".red());
        return;
    }
    match spec.contains {
        Some(contains) => search_contains(filter, contains),
        None => search_regular(filter),
    }
}

fn search_regular(filter: Filter) {
    let lexicon = Lexicon::load();

    lexicon
        .entries()
        .filter(|entry| filter.matches(entry))
        .for_each(|entry| {
            let word = match lexicon.rate(entry) {
                Popularity::Low => entry.word().to_owned(),
                Popularity::Medium => entry.word().yellow().to_string(),
                Popularity::High => entry.word().green().to_string(),
            };
            println!("{}", word);
        });
}

fn search_contains(filter: Filter, contains: String) {
    let lexicon = Lexicon::load();
    let sorted = SortedLetters::from_word(&contains);

    lexicon
        .entries()
        .filter(|entry| filter.matches(entry))
        .for_each(|entry| {
            let remaining =
                highlight_popular_words(&lexicon, entry.without_letters_in(&sorted).unwrap());
            let word = entry.word();
            println!("{contains} + {remaining} = {word}");
        });
}

fn highlight_popular_words(lexicon: &Lexicon, letters: SortedLetters) -> String {
    let ranked = solve_anagram(lexicon, &letters);
    match ranked.quality {
        lexi::Quality::NotWord | lexi::Quality::NotPopular => ranked.word,
        lexi::Quality::LessPopular => ranked.word.yellow().to_string(),
        lexi::Quality::VeryPopular => ranked.word.green().to_string(),
    }
}

pub fn gen_completions() {
    use clap_complete::shells::Fish;

    let cmd = &mut Cmdline::command();
    clap_complete::generate(Fish, cmd, cmd.get_name().to_owned(), &mut stdout());
}

#[derive(Debug, Parser)]
struct Cmdline {
    #[clap(subcommand)]
    command: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    Server(ServerOpts),
    Search(FilterSpec),
    Grep(GrepSpec),
    Completions,
}

#[derive(Debug, Parser)]
pub struct ServerOpts {
    #[clap(short, long)]
    addr: Option<String>,
}

#[derive(Debug, Parser, Default)]
#[clap(group(
            ArgGroup::new("filter")
                .required(true)
                .args(&["contains", "contained"]),
        ))]
pub struct FilterSpec {
    #[clap(long, short)]
    length: Option<LengthRange>,

    #[clap(long, short)]
    exclude_letters: Option<String>,

    #[clap(long, short)]
    include_letters: Option<String>,

    #[clap(long, short = '1')]
    one_word: bool,

    #[clap(long)]
    contains: Option<String>,

    #[clap(long)]
    contained: Option<String>,
}

impl FilterSpec {
    fn to_filter(&self) -> Filter {
        FilterBuilder::new()
            .length(self.length)
            .exclude_letters(self.exclude_letters.as_deref())
            .include_letters(self.include_letters.as_deref())
            .single_word(self.one_word.then_some(true))
            .contains(self.contains.as_deref())
            .contained(self.contained.as_deref())
            .build()
    }
}

#[derive(Debug, Parser)]
pub struct GrepSpec {
    #[clap(long, short = 'i')]
    case_insensitive: bool,

    regex: String,
}
