use clap::{App, Arg};
use std::process::Command;
use std::string::FromUtf8Error;
use std::{thread, time};

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
    let command_complete = command.join(" ");

    loop {
        match output_command(&command) {
            Ok(output) => format_output(&output, &command_complete, interval),
            Err(why) => format_output(
                &format!("Error reading output: {}", why),
                &command_complete,
                interval,
            ),
        }
        thread::sleep(time::Duration::from_secs(interval));
    }
}

fn format_output(output: &String, command_complete: &String, interval: u64) {
    print!("\x1B[2J\x1B[1;1H");

    println!(
        "Executing command: \"{}\" every {} seconds",
        command_complete, interval
    );
    println!("");
    println!("{}", output)
}

fn output_command(command: &Vec<String>) -> Result<String, FromUtf8Error> {
    let output = Command::new("powershell")
        .arg("-Command")
        .args(command)
        .output()
        .expect("failed to execute process");
    return String::from_utf8(output.stdout);
}
