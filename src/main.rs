extern crate chrono;
#[macro_use]
extern crate clap;
extern crate colored;
extern crate rand;
extern crate regex;
extern crate textwrap;

use chrono::prelude::*;
use clap::{App, AppSettings, Arg};
use colored::*;
use regex::Regex;
use textwrap::wrap_iter;

use minute::get_minute;

mod minute;

fn main() {
    let matches = App::new("Litime")
        .version(crate_version!())
        .setting(AppSettings::ColoredHelp)
        .about("Display a timestamp with a literature quote. By default, the current time stamp is used.")
        .arg(Arg::with_name("time")
            .short("t")
            .long("time")
            .value_name("time")
            .help("A timestamp to get a quote for, for example 07:16")
            .validator(is_timestamp)
            .takes_value(true),
        )
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .value_name("width")
            .help("The max width of the quote")
            .default_value("80")
            .takes_value(true),
        )
        .get_matches();

    let local: DateTime<Local> = Local::now();
    let now = format!(
        "{:0width$}:{:0width$}",
        local.hour(),
        local.minute(),
        width = 2
    );

    let timestamp = matches.value_of("time").unwrap_or(&now);
    let width = value_t!(matches, "width", usize).unwrap_or_else(|e| e.exit());

    let minute = get_minute(timestamp);
    let result = format!(
        "{}{}{}",
        minute.start.bright_black(),
        minute.time.red(),
        minute.end.bright_black()
    );

    let mut output = String::from("\n  \" ");

    for line in wrap_iter(&result, width) {
        output.push_str(&line);
        output.push_str("\n    ");
    }
    output.push('\n');
    output.push_str(format!("        {} - {}\n", minute.author, minute.title).as_str());
    print!("{}", output);
}

fn is_timestamp(val: String) -> Result<(), String> {
    let re = Regex::new(r"^([01][0-9]|2[0-3]):[0-5][0-9]$").unwrap();
    if re.is_match(&val) {
        Ok(())
    } else {
        Err(String::from(
            "the value must be a valid 24-hour timestamp in the format HH:MM",
        ))
    }
}
