mod models;
mod utils;

use std::error::Error;

use chrono::{NaiveDate, Utc};
use clap::Parser;
use models::args::CliArgs;

use crate::models::time::TimeDifferential;

const DATE_FORMAT: &str = "%m-%d-%Y";
// const SWHEN_FILE: &str = "swhen";

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let now = Utc::now();
    let date = NaiveDate::parse_from_str(&args.date, DATE_FORMAT)?;

    let ms_diff = now
        .date_naive()
        .signed_duration_since(date)
        .num_milliseconds();

    let diff = TimeDifferential::new(ms_diff);

    println!("{:?}", diff.to_string());

    Ok(())
}
