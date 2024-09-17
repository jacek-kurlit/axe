use axe::arg_parser;
use clap::Parser;
use std::io::{self, BufRead};
use std::{ffi::OsString, os::unix::ffi::OsStringExt, process::Command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("Run command for each entry of arguments"))]
struct Cli {
    #[arg(default_value = "echo")]
    cmd: String,
    template_args: Vec<String>,
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

//FIXME:template_args may be empty, it means that we should append all args as last argument
fn prepare_entries(cli: &Cli) -> Vec<Vec<String>> {
    let mut entries = Vec::new();
    let stdin_lines = read_input_lines();
    //FIXME: handle error
    let args_resolver = arg_parser::ArgumentResolver::new(&cli.template_args).unwrap();

    for stdin_entry in load_stdin_entries(stdin_lines, &cli.entries_separator) {
        let input_args = stdin_entry
            .split(&cli.args_separator)
            .collect::<Vec<&str>>();
        //FIXME: handle error
        //we may add flag to choose how to behave on error like:
        //panic and break
        //replace invalid value withempty string
        //ignore failed entry and continue with others
        let entry = args_resolver.resolve(input_args).unwrap();
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

fn load_stdin_entries(stdin_lines: Vec<String>, entries_separator: &str) -> Vec<String> {
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
        let actual = load_stdin_entries(stdin_lines, "\n");
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
        let actual = load_stdin_entries(stdin_lines.clone(), ";");
        assert_eq!(expected, actual);
    }
}
