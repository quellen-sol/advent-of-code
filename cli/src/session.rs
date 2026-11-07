use std::{env, fs, io::Write, os::unix::fs::OpenOptionsExt, os::unix::fs::PermissionsExt, path::Path};

use anyhow::{Context, Result, bail};

/// Same name as the CI secret, so one variable works everywhere.
const KEY: &str = "ADVENT_SESSION_KEY";
const ENV_FILE: &str = ".env";

pub struct Token {
    pub value: String,
    pub source: &'static str,
}

/// Looks up the token, preferring a real environment variable over `.env`.
pub fn load(root: &Path) -> Option<Token> {
    if let Some(value) = env::var(KEY).ok().filter(|value| !value.is_empty()) {
        return Some(Token {
            value,
            source: "environment",
        });
    }

    dotenvy::from_path_iter(root.join(ENV_FILE))
        .ok()?
        .flatten()
        .find(|(key, value)| key == KEY && !value.is_empty())
        .map(|(_, value)| Token {
            value,
            source: ENV_FILE,
        })
}

pub fn save(root: &Path, raw_token: &str) -> Result<()> {
    let token = normalize(raw_token);
    if token.is_empty() {
        bail!("token is empty");
    }

    let path = root.join(ENV_FILE);
    let existing = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(err).context("couldn't read .env"),
    };

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)
        .context("couldn't open .env for writing")?;
    // `mode` only applies when the file is created, so tighten an existing one.
    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    file.write_all(upsert(&existing, KEY, token).as_bytes())
        .context("couldn't write .env")
}

/// Shows just enough to tell two tokens apart without exposing a usable one.
pub fn mask(token: &str) -> String {
    let tail: String = {
        let chars: Vec<char> = token.chars().collect();
        chars[chars.len().saturating_sub(4)..].iter().collect()
    };
    format!("...{tail} ({} chars)", token.chars().count())
}

/// Accepts a token copied along with its cookie name.
fn normalize(raw: &str) -> &str {
    let trimmed = raw.trim();
    trimmed.strip_prefix("session=").unwrap_or(trimmed)
}

fn upsert(contents: &str, key: &str, value: &str) -> String {
    let entry = format!("{key}={value}");
    let prefix = format!("{key}=");

    let mut found = false;
    let mut lines: Vec<&str> = contents
        .lines()
        .map(|line| {
            if line.trim_start().starts_with(&prefix) {
                found = true;
                entry.as_str()
            } else {
                line
            }
        })
        .collect();
    if !found {
        lines.push(&entry);
    }

    lines.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upsert_replaces_the_key_and_keeps_everything_else() {
        let before = "ADVENT_SESSION_KEY=old\nADVENT_URL=https://example.com\n";
        assert_eq!(
            upsert(before, KEY, "new"),
            "ADVENT_SESSION_KEY=new\nADVENT_URL=https://example.com\n"
        );
    }

    #[test]
    fn upsert_appends_when_the_key_is_missing() {
        assert_eq!(upsert("OTHER=1\n", KEY, "new"), "OTHER=1\nADVENT_SESSION_KEY=new\n");
        assert_eq!(upsert("", KEY, "new"), "ADVENT_SESSION_KEY=new\n");
    }

    #[test]
    fn upsert_does_not_match_keys_that_share_a_prefix() {
        assert_eq!(
            upsert("ADVENT_SESSION_KEY_OLD=x\n", KEY, "new"),
            "ADVENT_SESSION_KEY_OLD=x\nADVENT_SESSION_KEY=new\n"
        );
    }

    #[test]
    fn normalize_strips_whitespace_and_cookie_name() {
        assert_eq!(normalize("  abc123\n"), "abc123");
        assert_eq!(normalize("session=abc123"), "abc123");
    }

    #[test]
    fn mask_never_reveals_more_than_the_tail() {
        assert_eq!(mask("abcdefgh"), "...efgh (8 chars)");
        assert_eq!(mask("ab"), "...ab (2 chars)");
    }

    #[test]
    fn save_round_trips_and_locks_down_permissions() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(ENV_FILE);
        fs::write(&path, "ADVENT_URL=keep-me\n").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

        save(dir.path(), " session=token123\n").unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        assert_eq!(contents, "ADVENT_URL=keep-me\nADVENT_SESSION_KEY=token123\n");
        let mode = fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
