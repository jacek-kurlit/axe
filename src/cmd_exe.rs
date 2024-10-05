use std::process::Command;

use crate::cli::Cli;

pub fn execute_cmd(cli: &Cli, cmd_args: Vec<String>) {
    if cli.debug {
        println!("{} {}", cli.cmd, cmd_args.join(" "));
    } else {
        run_command(cli, &cmd_args);
    };
}

fn run_command(cli: &Cli, cmd_args: &[String]) {
    let handle = Command::new(&cli.cmd).args(cmd_args).spawn();
    match handle {
        Ok(child) => {
            if let Err(error) = child.wait_with_output() {
                eprintln!("Failed to await for command to finish: {}", error);
            }
        }
        Err(error) => eprintln!("Failed to spawn command: {}", error),
    }
}
