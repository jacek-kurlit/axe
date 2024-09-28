use std::process::Command;

use crate::cli::Cli;

pub fn execute_cmd(cli: &Cli, cmd_args: Vec<String>) {
    let output = if cli.debug {
        print_command(cli, &cmd_args)
    } else {
        run_command(cli, &cmd_args)
    };
    match output {
        Ok(output) => println!("{}", output.trim()),
        Err(error) => eprintln!("Failed to execute command:\n{}", error),
    }
}

fn print_command(cli: &Cli, cmd_args: &[String]) -> Result<String, String> {
    Ok(format!("{} {}", cli.cmd, cmd_args.join(" ")))
}

fn run_command(cli: &Cli, cmd_args: &[String]) -> Result<String, String> {
    let output = Command::new(&cli.cmd)
        .args(cmd_args)
        .output()
        .expect("Failed to execute command");
    match output.status.success() {
        true => String::from_utf8(output.stdout)
            .map_err(|e| format!("Invalid UTF-8 command output: {}", e)),
        false => Err(String::from_utf8(output.stderr).unwrap()),
    }
}
