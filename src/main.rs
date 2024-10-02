use axe_cli::arg_resolver::resolve_cmd_args;
use axe_cli::cli::Cli;
use axe_cli::cmd_exe::execute_cmd;
use axe_cli::stdin::read_entries;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let stdin_entries = read_entries(&cli.args_file, &cli.entries);
    let resolved_cmd_args = resolve_cmd_args(stdin_entries, &cli);

    for cmd_args in resolved_cmd_args {
        execute_cmd(&cli, cmd_args);
    }
}
