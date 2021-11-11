use anyhow::Result;
use rayon::prelude::*;
use std::io::Read;
use std::{fs::File, io::BufReader};
use unicode_normalization::{self, UnicodeNormalization};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

struct Trie {
    ch: char,
    parent: *const Trie,
    children: std::collections::HashMap<char, *const Trie>,
}

impl Trie {
    fn from(&self, lines: impl Iterator<Item = String>) {
        lines.for_each(|l| self.insert_line(&l));
    }
    fn insert_line(&self, line: &str) {
        for ch in line.chars() {}
    }
}

fn read<'a, P: AsRef<std::path::Path>>(p: P) -> Result<String> {
    let mut buf = String::new();
    zstd::Decoder::new(BufReader::new(File::open(p)?))?.read_to_string(&mut buf)?;

    Ok(buf.chars().nfc().stream_safe().collect::<String>())
}

pub fn foo(pattern: &str) -> Result<Vec<String>> {
    let zst = read("home.zst")?;
    let matcher = SkimMatcherV2::default();
    let mut raw = zst
        .par_lines()
        .filter_map(|l| matcher.fuzzy_match(l, pattern).map(|score| (score, l)))
        .into_par_iter()
        .collect::<Vec<_>>();

    raw.par_sort_by_key(|(score, _)| *score);
    let found = raw
        .par_iter()
        .map(|(_, s)| s.to_string())
        .collect::<Vec<_>>();
    Ok(found)
}

pub mod store {
    pub use super::foo;
}
