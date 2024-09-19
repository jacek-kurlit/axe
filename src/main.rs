use axe::arg_parser::resolve_cmd_args;
use axe::cli::Cli;
use axe::cmd_exe::execute_cmd;
use axe::stdin::read_stdin_entries;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let stdin_entries = read_stdin_entries(&cli.entries_separator);
    let resolved_cmd_args = resolve_cmd_args(stdin_entries, &cli);

    for cmd_args in resolved_cmd_args {
        execute_cmd(&cli, cmd_args);
    }
}
