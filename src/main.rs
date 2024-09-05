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
    let input_lines = read_input_lines();
    let entries = prepare_entries(&cli, input_lines);

    for entry in entries {
        execute_cmd(&cli, entry);
    }
}

fn read_input_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.expect("Could not read line from standard in"));
    }
    lines
}

fn prepare_entries(cli: &Cli, input_lines: Vec<String>) -> Vec<Vec<String>> {
    let mut entries = Vec::new();
    for line in input_lines {
        let input_args = line.split(&cli.args_separator).collect::<Vec<&str>>();
        let entry = cli
            .initial_args
            .iter()
            .map(|a| a.as_ref())
            .chain(input_args)
            //TODO: cloning
            .map(|a| a.to_string())
            .collect::<Vec<String>>();
        entries.push(entry);
    }
    entries
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
