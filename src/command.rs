use std::process::Command;

pub fn output_command(command: &Vec<String>) -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .args(command)
        .output();
    return match output {
        Ok(output) => {
            format!(
                "{}\n{}",
                parse_string(&output.stdout),
                parse_string(&output.stderr)
            )
        }
        Err(why) => format!("Error executing command: {}", why),
    };
}

fn parse_string(output: &Vec<u8>) -> String {
    return String::from_utf8(output.to_vec()).unwrap();
}
