mod models;
mod utils;

use clap::Parser;
use models::{args::CliArgs, error::Error};

const DATE_FORMAT: &str = "%Y-%m-%d";
const SWHEN_FILE: &str = "swhen";

fn main() -> Result<(), Error> {
    let args = CliArgs::parse();

    println!("args = {:?}", args);

    // let now = Utc::now();

    Ok(())
}
