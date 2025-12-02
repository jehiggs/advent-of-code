use clap::Parser;
use std::env;
use std::error::Error;
use std::fmt::{Debug, Display};
use std::fs;
use std::path::PathBuf;
use ureq::get;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    year: u16,
    day: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut dir = PathBuf::from(format!("./{}/day-{}/", args.year, args.day));
    if !dir.try_exists()? {
        return Err(error("Directory for year and day does not exist."));
    }
    let session_cookie = env::var("AOC_SESSION")?;
    let url = format!(
        "https://adventofcode.com/{}/day/{}/input",
        args.year, args.day
    );
    // Assume the input is short enough that reading it into a string will be fine.
    let input = get(url)
        .header("Cookie", format!("session={session_cookie}"))
        .call()?
        .body_mut()
        .read_to_string()?;

    // Assume all write paths are relative to the workspace root.
    dir.push("input.txt");
    fs::write(dir, input)?;
    Ok(())
}

fn error(msg: &'static str) -> Box<dyn Error> {
    Box::new(AocError { reason: msg })
}

#[derive(Debug)]
struct AocError {
    reason: &'static str,
}

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for AocError {}
