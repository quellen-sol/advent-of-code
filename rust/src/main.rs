// `utils` is a toolbox that no single day uses all of, and these lints would
// otherwise be replayed above the answer on every run.
#![allow(dead_code, mismatched_lifetime_syntaxes)]

use std::{process, time::Duration};

use clap::Parser;

mod defs;
mod timing;
mod utils;
mod years;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    year: u16,

    #[clap(long)]
    day: u8,

    #[clap(long)]
    input: String,

    /// Seconds to spend measuring setup and each part. Runs each one once if left out.
    #[clap(long, default_value_t = 0.0)]
    bench: f64,
}

fn main() {
    let args = Args::parse();
    let budget = Duration::from_secs_f64(args.bench);

    if !years::run(args.year, args.day, &args.input, budget) {
        eprintln!("No Rust solution for {} day {}", args.year, args.day);
        process::exit(1);
    }
}
