mod date;
mod input;
mod lang;
mod registry;
mod session;

use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};

use input::Input;
use lang::Lang;

#[derive(Parser)]
#[command(name = "aoc", about = "Run and scaffold Advent of Code solutions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Build and run a solution
    Run {
        #[command(flatten)]
        target: Target,

        /// Use this file instead of the puzzle input, e.g. a sample
        #[arg(short, long)]
        input: Option<PathBuf>,
    },
    /// Build a solution and measure it over repeated runs
    Bench {
        #[command(flatten)]
        target: Target,

        /// Use this file instead of the puzzle input, e.g. a sample
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Seconds to spend measuring setup and each part
        #[arg(short, long, default_value_t = 1.0, value_parser = parse_seconds)]
        time: f64,
    },
    /// Create a solution from its template and download the input
    New {
        #[command(flatten)]
        target: Target,
    },
    /// Manage the adventofcode.com session token
    Session {
        #[command(subcommand)]
        command: SessionCommand,
    },
}

#[derive(Subcommand)]
enum SessionCommand {
    /// Save the token to .env, prompting for it if not given
    Set { token: Option<String> },
    /// Show where the token comes from, masked
    Show,
}

#[derive(Args)]
struct Target {
    #[arg(value_enum, default_value_t = Lang::Rust)]
    lang: Lang,

    /// Defaults to the current year on AoC's clock (UTC-5)
    #[arg(short, long)]
    year: Option<u16>,

    /// Defaults to the current day of the month on AoC's clock (UTC-5)
    #[arg(short, long)]
    day: Option<u8>,
}

fn parse_seconds(value: &str) -> Result<f64, String> {
    match value.parse() {
        Ok(seconds) if seconds > 0.0 && f64::is_finite(seconds) => Ok(seconds),
        _ => Err("expected a positive number of seconds".to_string()),
    }
}

fn main() {
    if let Err(err) = dispatch() {
        eprintln!("error: {err:#}");
        process::exit(1);
    }
}

fn dispatch() -> Result<()> {
    let cli = Cli::parse();
    let root = find_root()?;

    match cli.command {
        Command::Run { target, input } => run(&root, target, input, None),
        Command::Bench {
            target,
            input,
            time,
        } => run(&root, target, input, Some(time)),
        Command::New { target } => new(&root, target),
        Command::Session { command } => match command {
            SessionCommand::Set { token } => set_session(&root, token),
            SessionCommand::Show => show_session(&root),
        },
    }
}

/// Walks up from the working directory so `aoc` works from any subdirectory.
fn find_root() -> Result<PathBuf> {
    let cwd = env::current_dir().context("couldn't read the working directory")?;
    cwd.ancestors()
        .find(|dir| dir.join("cli/Cargo.toml").is_file() && dir.join("rust/Cargo.toml").is_file())
        .map(Path::to_path_buf)
        .context("not inside the advent-of-code repo")
}

fn run(
    root: &Path,
    target: Target,
    input_override: Option<PathBuf>,
    bench_seconds: Option<f64>,
) -> Result<()> {
    let lang = target.lang;
    let (year, day) = date::resolve(target.year, target.day, date::now())?;

    registry::sync(root, lang)?;

    if !lang.solution_path(root, year, day).exists() {
        bail!(
            "no {name} solution for {year} day {day}; create it with `aoc new {name} -y {year} -d {day}`",
            name = lang.name()
        );
    }

    let input = match input_override {
        // Absolute, because the runner's working directory is the language dir.
        Some(path) => fs::canonicalize(&path)
            .with_context(|| format!("couldn't find input file {}", path.display()))?,
        None => match input::ensure(root, year, day)? {
            Input::Ready(path) => path,
            Input::Locked => {
                bail!("{year} day {day} hasn't unlocked yet; use -i to run against a sample")
            }
        },
    };

    let status = lang
        .command(root, year, day, &input, bench_seconds)
        .status()
        .with_context(|| format!("couldn't start the {} runner", lang.name()))?;

    process::exit(status.code().unwrap_or(1));
}

fn new(root: &Path, target: Target) -> Result<()> {
    let lang = target.lang;
    let (year, day) = date::resolve(target.year, target.day, date::now())?;

    let path = lang.solution_path(root, year, day);
    if path.exists() {
        println!("Already exists: {}", relative(root, &path));
    } else {
        scaffold(root, lang, year, day, &path)?;
        println!("Created: {}", relative(root, &path));
    }

    registry::sync(root, lang)?;

    // The solution is in place by now, so a failure here is safe to fix and
    // retry: `aoc new` won't touch the existing file.
    match input::ensure(root, year, day)? {
        Input::Ready(path) => println!("Input: {}", relative(root, &path)),
        Input::Locked => {
            println!("Input: {year} day {day} hasn't unlocked yet, run this again once it has")
        }
    }

    Ok(())
}

fn scaffold(root: &Path, lang: Lang, year: u16, day: u8, path: &Path) -> Result<()> {
    let template_path = lang.template_path(root);
    let template = fs::read_to_string(&template_path)
        .with_context(|| format!("couldn't read {}", template_path.display()))?;

    let dir = path.parent().expect("solution path has a parent");
    fs::create_dir_all(dir).with_context(|| format!("couldn't create {}", dir.display()))?;
    fs::write(path, lang::render_template(&template, year, day))
        .with_context(|| format!("couldn't write {}", path.display()))
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root).unwrap_or(path).display().to_string()
}

fn set_session(root: &Path, token: Option<String>) -> Result<()> {
    let token = match token {
        Some(token) => token,
        None => rpassword::prompt_password("Session token: ")
            .context("couldn't read the token; pass it as an argument instead")?,
    };

    session::save(root, &token)?;
    println!("Saved to .env");
    Ok(())
}

fn show_session(root: &Path) -> Result<()> {
    match session::load(root) {
        Some(token) => println!("{} from {}", session::mask(&token.value), token.source),
        None => println!("No session token set; add one with `aoc session set`"),
    }
    Ok(())
}
