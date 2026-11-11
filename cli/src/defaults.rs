//! Remembers which solution `aoc run` and `aoc bench` should use when the
//! language, -y or -d is left out, so working through an old year doesn't mean
//! retyping it each time.

use std::{fmt::Write, fs, path::Path};

use anyhow::Context;
use clap::ValueEnum;

use crate::lang::Lang;

const FILE: &str = ".aoc-defaults";

/// A language, year and day: everything that picks out one solution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Solution {
    pub lang: Lang,
    pub year: u16,
    pub day: u8,
}

#[derive(Default, Debug, PartialEq)]
pub struct Defaults {
    /// Set by `aoc new`.
    pub run: Option<Solution>,
    /// Set by `aoc new` and `aoc run`.
    pub bench: Option<Solution>,
}

impl Defaults {
    /// A missing or unreadable file just means nothing is remembered yet.
    pub fn load(root: &Path) -> Self {
        fs::read_to_string(root.join(FILE))
            .map(|contents| parse(&contents))
            .unwrap_or_default()
    }

    /// Failing to remember a solution shouldn't stop the command that chose
    /// it, so this only warns.
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
        let solution = parse_solution(value.trim());
        match key.trim() {
            "run" => defaults.run = solution,
            "bench" => defaults.bench = solution,
            _ => {}
        }
    }
    defaults
}

/// Reads the `py 2022/8` form that `render` writes.
fn parse_solution(value: &str) -> Option<Solution> {
    let (lang, puzzle) = value.split_once(' ')?;
    let (year, day) = puzzle.trim().split_once('/')?;
    Some(Solution {
        lang: Lang::from_str(lang, false).ok()?,
        year: year.parse().ok()?,
        day: day.parse().ok()?,
    })
}

fn render(defaults: &Defaults) -> String {
    let mut out = String::new();
    for (key, solution) in [("run", defaults.run), ("bench", defaults.bench)] {
        if let Some(Solution { lang, year, day }) = solution {
            writeln!(out, "{key}={} {year}/{day}", lang.name())
                .expect("writing to a String can't fail");
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
            run: Some(Solution {
                lang: Lang::Python,
                year: 2022,
                day: 8,
            }),
            bench: Some(Solution {
                lang: Lang::Rust,
                year: 2023,
                day: 14,
            }),
        };

        defaults.save(dir.path());

        assert_eq!(
            fs::read_to_string(dir.path().join(FILE)).unwrap(),
            "run=py 2022/8\nbench=rust 2023/14\n"
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
        let parsed = parse("run=2022/8\nbench=go 2022/8\nnonsense\nother=py 2021/1\n");
        assert_eq!(
            parsed,
            Defaults {
                run: None,
                bench: Some(Solution {
                    lang: Lang::Go,
                    year: 2022,
                    day: 8,
                }),
            }
        );
    }
}
