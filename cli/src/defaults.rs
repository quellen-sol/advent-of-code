//! Remembers which puzzle `aoc run` and `aoc bench` should use when -y or -d is
//! left out, so working through an old year doesn't mean retyping it each time.

use std::{fmt::Write, fs, path::Path};

use anyhow::Context;

const FILE: &str = ".aoc-defaults";

/// A year and day.
pub type Puzzle = (u16, u8);

#[derive(Default, Debug, PartialEq)]
pub struct Defaults {
    /// Set by `aoc new`.
    pub run: Option<Puzzle>,
    /// Set by `aoc new` and `aoc run`.
    pub bench: Option<Puzzle>,
}

impl Defaults {
    /// A missing or unreadable file just means nothing is remembered yet.
    pub fn load(root: &Path) -> Self {
        fs::read_to_string(root.join(FILE))
            .map(|contents| parse(&contents))
            .unwrap_or_default()
    }

    /// Failing to remember a puzzle shouldn't stop the command that chose it,
    /// so this only warns.
    pub fn save(&self, root: &Path) {
        let result = fs::write(root.join(FILE), render(self))
            .with_context(|| format!("couldn't save {FILE}"));
        if let Err(err) = result {
            eprintln!("warning: {err:#}");
        }
    }
}

fn parse(contents: &str) -> Defaults {
    let mut defaults = Defaults::default();
    for line in contents.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let puzzle = parse_puzzle(value.trim());
        match key.trim() {
            "run" => defaults.run = puzzle,
            "bench" => defaults.bench = puzzle,
            _ => {}
        }
    }
    defaults
}

fn parse_puzzle(value: &str) -> Option<Puzzle> {
    let (year, day) = value.split_once('/')?;
    Some((year.parse().ok()?, day.parse().ok()?))
}

fn render(defaults: &Defaults) -> String {
    let mut out = String::new();
    for (key, puzzle) in [("run", defaults.run), ("bench", defaults.bench)] {
        if let Some((year, day)) = puzzle {
            writeln!(out, "{key}={year}/{day}").expect("writing to a String can't fail");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_through_the_file() {
        let dir = tempfile::tempdir().unwrap();
        let defaults = Defaults {
            run: Some((2022, 8)),
            bench: Some((2023, 14)),
        };

        defaults.save(dir.path());

        assert_eq!(
            fs::read_to_string(dir.path().join(FILE)).unwrap(),
            "run=2022/8\nbench=2023/14\n"
        );
        assert_eq!(Defaults::load(dir.path()), defaults);
    }

    #[test]
    fn missing_file_remembers_nothing() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(Defaults::load(dir.path()), Defaults::default());
    }

    #[test]
    fn skips_whatever_it_cant_read() {
        let parsed = parse("run=2022\nbench=2022/8\nnonsense\nother=2021/1\n");
        assert_eq!(
            parsed,
            Defaults {
                run: None,
                bench: Some((2022, 8)),
            }
        );
    }
}
