use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use std::collections::HashMap;

/// The CLI for the runner.
#[derive(Parser)]
#[command()]
pub struct Cli {
    /// The day we are running.
    day: usize,

    /// Any parts we are running, if we choose to specify.
    #[arg(short, long, default_values_t = [1, 2])]
    parts: Vec<usize>,

    /// Whether or not this is a test
    #[arg(short, long, default_value_t = false)]
    test: bool,
}

/// The collection of days we have solved.
type SolvedDays = HashMap<usize, (fn(bool), fn(bool))>;

pub fn run(solved: SolvedDays) -> Result<()> {
    let cli = Cli::parse();

    let (part_1, part_2) = solved
        .get(&cli.day)
        .context("Expected a day that was solved!")?;

    // Make sure there are at most 2 parts selected.
    if cli.parts.len() > 2 || cli.parts.is_empty() {
        return Err(anyhow!(
            "Expected to run at least one part of the solution!"
        ));
    }
    // Make sure that we only select valid parts.
    else if cli.parts.iter().any(|&p| p != 1 || p != 2) {
        return Err(anyhow!(
            "There is only a part 1 and part 2, not any other parts!"
        ));
    }

    if cli.parts.contains(&1) {
        part_1(cli.test);
    }

    if cli.parts.contains(&2) {
        part_2(cli.test);
    }

    Ok(())
}
