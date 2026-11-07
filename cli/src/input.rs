use std::{
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{Context, Result, bail};

use crate::{date, session};

/// AoC asks automated tools to identify themselves. The repo link gives them
/// somewhere to look if this ever misbehaves.
const USER_AGENT: &str = "github.com/quellen-sol/advent-of-code via aoc-cli";

pub enum Input {
    Ready(PathBuf),
    /// The puzzle hasn't unlocked, so no request was made.
    Locked,
}

pub fn path(root: &Path, year: u16, day: u8) -> PathBuf {
    root.join("inputs")
        .join(year.to_string())
        .join(format!("{day}.txt"))
}

/// Returns the input for a puzzle, downloading it only if it isn't on disk.
pub fn ensure(root: &Path, year: u16, day: u8) -> Result<Input> {
    let path = path(root, year, day);
    if path.exists() {
        return Ok(Input::Ready(path));
    }

    if !date::is_unlocked(year, day, date::now()) {
        return Ok(Input::Locked);
    }

    let token = session::load(root)
        .context("no session token to download the input with; set one with `aoc session set`")?;
    let text = download(&token.value, year, day)?;

    let dir = path.parent().expect("input path has a parent");
    fs::create_dir_all(dir).with_context(|| format!("couldn't create {}", dir.display()))?;
    fs::write(&path, text).with_context(|| format!("couldn't write {}", path.display()))?;
    eprintln!("Downloaded input for {year} day {day}");

    Ok(Input::Ready(path))
}

fn download(token: &str, year: u16, day: u8) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let mut response = ureq::get(&url)
        .header("Cookie", format!("session={token}"))
        .header("User-Agent", USER_AGENT)
        .config()
        .http_status_as_error(false)
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .call()
        .with_context(|| format!("couldn't reach {url}"))?;

    let status = response.status().as_u16();
    let body = response
        .body_mut()
        .read_to_string()
        .context("couldn't read the response body")?;

    parse_response(status, &body, year, day)
}

/// AoC sends its error messages as plain text, so anything but a 200 has to be
/// rejected here or the message gets saved as if it were puzzle input.
fn parse_response(status: u16, body: &str, year: u16, day: u8) -> Result<String> {
    match status {
        // Only the end is trimmed: some inputs start with meaningful spaces.
        200 => Ok(body.trim_end().to_string()),
        400 | 500 => bail!(
            "adventofcode.com rejected the session token (HTTP {status}); \
             it may have expired, so update it with `aoc session set`"
        ),
        404 => bail!("{year} day {day} isn't available; it may not exist or may not be unlocked"),
        _ => bail!("unexpected HTTP {status} from adventofcode.com: {}", body.trim()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_inputs_by_year_then_day() {
        assert_eq!(
            path(Path::new("/repo"), 2026, 5),
            PathBuf::from("/repo/inputs/2026/5.txt")
        );
    }

    #[test]
    fn success_trims_only_the_end() {
        let parsed = parse_response(200, "    [D]\n1 2 3\n\n", 2022, 5).unwrap();
        assert_eq!(parsed, "    [D]\n1 2 3");
    }

    #[test]
    fn error_bodies_never_become_input() {
        let unlock_message = "Please don't repeatedly request this endpoint before it unlocks!";
        for status in [400, 404, 429, 500, 503] {
            assert!(parse_response(status, unlock_message, 2024, 6).is_err());
        }
    }

    #[test]
    fn rejected_session_points_at_the_fix() {
        let err = parse_response(400, "Please log in", 2024, 1).unwrap_err();
        assert!(err.to_string().contains("aoc session set"));
    }
}
