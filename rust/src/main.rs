// `utils` is a toolbox that no single day uses all of, and these lints would
// otherwise be replayed above the answer on every run.
#![allow(dead_code, mismatched_lifetime_syntaxes)]

use std::process;

use clap::Parser;

mod defs;
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
}

fn main() {
    let args = Args::parse();

    if !years::run(args.year, args.day, &args.input) {
        eprintln!("No Rust solution for {} day {}", args.year, args.day);
        process::exit(1);
    }
}
