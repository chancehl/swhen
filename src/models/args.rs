use clap::{command, Parser};

/// A program to determine how long since a date has passed
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// The date to check against
    #[arg(short = 'd', long = "date")]
    pub date: Option<String>,

    /// The alias to save the date as
    #[arg(short = 'a', long = "alias")]
    pub alias: Option<String>,
}
