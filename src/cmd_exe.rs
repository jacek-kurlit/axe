use std::{ffi::OsString, os::unix::ffi::OsStringExt, process::Command};

use crate::cli::Cli;

pub fn execute_cmd(cli: &Cli, cmd_args: Vec<String>) {
    let output = Command::new(&cli.cmd)
        .args(cmd_args)
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
