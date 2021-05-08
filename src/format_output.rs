pub use crate::terminal_utils::{clear_screen, TERMINAL_COLOR_ORANGE, TERMINAL_COLOR_RESET};

pub fn format_output(
    output: &String,
    previous_output: &String,
    command_complete: &String,
    interval: u64,
    with_difference: bool,
) {
    if with_difference {
        format_output_with_difference(&output, &previous_output, &command_complete, interval);
    } else {
        format_output_without_difference(&output, &command_complete, interval);
    }
}

fn format_output_without_difference(output: &String, command_complete: &String, interval: u64) {
    clear_screen();

    println!(
        "Executing command: \"{}\" every {} seconds",
        command_complete, interval
    );
    println!("");
    println!("{}", output)
}

fn format_output_with_difference(
    output: &String,
    previous_output: &String,
    command_complete: &String,
    interval: u64,
) {
    clear_screen();
    let diffed_output = output
        .lines()
        .enumerate()
        .map(|(iterate_line, line)| {
            let previous_line = previous_output.lines().nth(iterate_line).unwrap_or("");
            color_string_diff(line, previous_line)
        })
        .collect::<Vec<String>>()
        .join("\n");

    println!(
        "Executing command: \"{}\" every {} seconds\n{}",
        command_complete, interval, diffed_output
    );
}

fn color_string_diff(string1: &str, string2: &str) -> String {
    let mut diffed_output = String::new();
    for (i, c) in string1.chars().enumerate() {
        if string2.chars().nth(i).unwrap_or('\0') != c {
            diffed_output.push_str(TERMINAL_COLOR_ORANGE);
        } else {
            diffed_output.push_str(TERMINAL_COLOR_RESET);
        }
        diffed_output.push(c);
    }
    diffed_output.push_str(TERMINAL_COLOR_RESET);
    return diffed_output;
}
