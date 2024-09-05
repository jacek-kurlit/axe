use clap::Parser;
use std::io::{self, BufRead};
use std::{ffi::OsString, os::unix::ffi::OsStringExt, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("Run command for each entry of arguments"))]
struct Cli {
    #[arg(default_value = "echo")]
    cmd: String,
    initial_args: Vec<String>,
    #[arg(short, long, default_value = " ")]
    args_separator: String,
    #[arg(short, long, default_value = "\n")]
    entries_separator: String,
}

fn main() {
    let cli = Cli::parse();
    let entries = prepare_entries(&cli);

    for entry in entries {
        execute_cmd(&cli, entry);
    }
}

fn prepare_entries(cli: &Cli) -> Vec<Vec<String>> {
    let mut entries = Vec::new();
    let stdin_lines = read_input_lines();
    for input_line in parse_as_input_entries(stdin_lines, &cli.entries_separator) {
        let input_args = input_line.split(&cli.args_separator).collect::<Vec<&str>>();
        let entry = cli
            .initial_args
            .iter()
            .map(|a| a.as_ref())
            .chain(input_args)
            //FIXME: find a way to avoid cloning
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        entries.push(entry);
    }
    entries
}

fn read_input_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.expect("Could not read line from standard in"));
    }
    lines
}

fn parse_as_input_entries(stdin_lines: Vec<String>, entries_separator: &str) -> Vec<String> {
    if entries_separator == "\n" {
        return stdin_lines;
    }
    stdin_lines
        .join("")
        .split(entries_separator)
        //FIXME: find a way to avoid cloning
        .map(|l| l.to_string())
        .collect()
}

fn execute_cmd(cli: &Cli, entry: Vec<String>) {
    println!("{} {:?}", cli.cmd, &entry);
    let output = Command::new(&cli.cmd)
        .args(entry)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        let stdout = OsString::from_vec(output.stdout);
        println!("{}", stdout.to_string_lossy().trim());
    } else {
        let stderr = OsString::from_vec(output.stderr);
        eprintln!("Command failed with error:\n{}", stderr.to_string_lossy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_stdio_lines_as_input_entries_for_new_line_separator() {
        let stdin_lines = vec!["a b c".to_string(), "d e f".to_string()];
        let expected = stdin_lines.clone();
        let actual = parse_as_input_entries(stdin_lines, "\n");
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_stdio_lines_as_input_entries_for_colon_separator() {
        let stdin_lines = vec!["a b c;d e f;".to_string(), "g h i;j k l".to_string()];
        let expected = vec![
            "a b c".to_string(),
            "d e f".to_string(),
            "g h i".to_string(),
            "j k l".to_string(),
        ];
        let actual = parse_as_input_entries(stdin_lines.clone(), ";");
        assert_eq!(expected, actual);
    }
}
