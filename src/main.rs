mod models;
mod utils;

use std::{env::temp_dir, error::Error};

use chrono::{NaiveDate, Utc};
use clap::Parser;
use models::args::CliArgs;
use regex::Regex;

use crate::{
    models::{alias::Alias, time::TimeDifferential},
    utils::file::{get_alias, save_alias},
};

const DATE_FORMAT: &str = "%m/%d/%Y";
const SWHEN_FILE: &str = "swhen";

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();

    let now = Utc::now();

    let date_regex = Regex::new("^\\d{2}/\\d{2}/\\d{4}$").unwrap();

    let aliases_loc = temp_dir().join(SWHEN_FILE);

    let date = if date_regex.is_match(&args.date) {
        NaiveDate::parse_from_str(&args.date, DATE_FORMAT)?
    } else {
        let alias = get_alias(&args.date, &aliases_loc)
            .expect("Could not locate aliases")
            .expect(&format!("Could not find alias {:?}", &args.date));

        NaiveDate::parse_from_str(&alias.date, DATE_FORMAT)?
    };

    if let Some(alias) = &args.alias {
        let new_alias = Alias::new(alias, &args.date);

        if let Err(err) = save_alias(new_alias, &aliases_loc) {
            println!("Failed to save alias (err = {:?}", err);
        } else {
            println!("Saved alias to {:?}", &aliases_loc);
        }
    }

    let ms_diff = now
        .date_naive()
        .signed_duration_since(date)
        .num_milliseconds()
        .abs();

    let diff = TimeDifferential::new(ms_diff);

    println!("{}", diff.to_string());

    Ok(())
}
