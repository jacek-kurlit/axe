use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

#[test]
fn should_resolve_args() {
    let output = execute_with(&["-d", "echo", "{2}-{1}-{0}"], "a b c\nd e f");
    assert_eq!(output, vec!("echo c-b-a", "echo f-e-d"));
}

#[test]
fn should_allow_to_change_args_separator() {
    let output = execute_with(&["-d", "-a=;", "echo", "{}"], "a;b;c\nd;e;f");
    assert_eq!(output, vec!("echo a b c", "echo d e f"));
}

#[test]
fn should_allow_to_change_entries_separator() {
    let output = execute_with(&["-d", "-e=|", "echo", "{}"], "a b c|d e f");
    assert_eq!(output, vec!("echo a b c", "echo d e f"));
}

#[test]
fn should_print_debug_instead_of_running_command() {
    let output = execute_with(&["-d", "echo", "{}"], "a b c\nd e f");
    assert_eq!(output, vec!("echo a b c", "echo d e f"));
}

#[test]
fn should_use_arg_file_instead_of_stdin() {
    let output = Command::new("cargo")
        .args([
            "run",
            "-q",
            "--",
            "-f",
            "tests/test_arg_file.txt",
            "echo",
            "{}",
        ])
        .output()
        .expect("Failed to execute cargo run");
    assert_eq!(read_output_lines(output), vec!["a b c", "d e f"]);
}

fn execute_with(args: &[&str], input: &str) -> Vec<String> {
    let mut cargo_handle = Command::new("cargo")
        .args(["run", "-q", "--"])
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute cargo run");
    cargo_handle
        .stdin
        .take()
        .expect("Cannot get stdin for cargo process")
        .write_all(input.as_bytes())
        .expect("Failed to write to cargo process stdin");
    let output = cargo_handle
        .wait_with_output()
        .expect("Failed to wait for cargo process");
    read_output_lines(output)
}

fn read_output_lines(output: Output) -> Vec<String> {
    String::from_utf8(output.stdout)
        .expect("Failed to convert output to string")
        .trim()
        .to_string()
        .lines()
        .map(|line| line.to_string())
        .collect()
}
