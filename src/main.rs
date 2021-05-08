use clap::{App, Arg};
use std::{thread, time};

mod command;
mod format_output;
mod terminal_utils;

pub use crate::command::output_command;
pub use crate::format_output::format_output;
pub use crate::terminal_utils::clear_screen;

fn main() {
    let matches = App::new("watch")
        .version("1.0")
        .author("Lukas Meisegeier <klg71@web.de>")
        .about("Invokes a command in specific interfalls and prints its stdout")
        .arg(
            Arg::new("interval")
                .short('n')
                .long("interval")
                .value_name("interval")
                .default_value("2")
                .about("Interval between function calls")
                .takes_value(true),
        )
        .arg(
            Arg::new("differences")
                .short('d')
                .long("differences")
                .value_name("differences")
                .about("Highlights changes")
                .takes_value(false),
        )
        .arg(
            Arg::new("command")
                .value_name("command")
                .about("command to run")
                .required(true)
                .multiple_occurrences(true)
                .multiple_values(true)
                .takes_value(true),
        )
        .get_matches();

    let command: Vec<String> = matches
        .values_of("command")
        .unwrap()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let interval: u64 = (matches.value_of("interval").unwrap())
        .parse::<u64>()
        .unwrap();
    let differences: bool = matches.is_present("differences");

    let command_complete = command.join(" ");

    let mut previous_output = String::new();

    loop {
        let output = output_command(&command);
        format_output(
            &output,
            &previous_output,
            &command_complete,
            interval,
            differences,
        );
        previous_output.clear();
        previous_output.push_str(&output);
        thread::sleep(time::Duration::from_secs(interval));
    }
}
