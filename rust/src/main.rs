//! GLOBALLY REPLACE THE YEAR

use std::process;

use clap::Parser;
use defs::Solution;

mod defs;
mod utils;
mod years;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    download_input_day: Option<u8>,

    #[clap(env = "ADVENT_SESSION_KEY")]
    advent_session_key: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    if let Some(day) = args.download_input_day {
        let session_key = args
            .advent_session_key
            .expect("ADVENT_SESSION_KEY not set for downloading input");
        let url = format!("https://adventofcode.com/{}/day/{}/input", 2025, day);
        let client = reqwest::Client::new();
        let result = client
            .get(url)
            .header("Cookie", format!("session={session_key}"))
            .send()
            .await
            .unwrap();
        let text = sanitize_downloaded_input(result.text().await.unwrap());
        std::fs::write(
            format!("./src/years/y{}/inputs/{}-input.txt", 2025, day),
            text,
        )
        .unwrap();
        println!("Downloaded input for day {day}");
        process::exit(0);
    }

    make_and_run_solution!(1, 2025);
}

pub fn sanitize_downloaded_input(input: String) -> String {
    input.trim().to_string()
}
