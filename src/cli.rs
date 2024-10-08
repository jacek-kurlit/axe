use std::path::PathBuf;

use clap::{arg, command, Args};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = Some("Run command for each entry of arguments"))]
pub struct Cli {
    #[arg(default_value = "echo")]
    pub cmd: String,
    /// Arguments templates that will be resolved and passed to cmd.
    /// Allowed values:
    /// - static text
    /// - {} - all input arguments
    /// - {<sep>} - all input arguments, each splittded by <sep> and all parts of split are taken
    /// - {<sep>y} - all input arguments, each splittded by <sep> and y-th part of split is taken
    /// - {x} - x-th argument
    /// - {x<sep>} - x-th argument splittded by <sep> and all parts of split are taken
    /// - {x<sep>y} - x-th argument splittded by <sep> and y-th part of split is taken.
    #[arg(verbatim_doc_comment, allow_hyphen_values = true)]
    pub args_templates: Vec<String>,
    /// Separator between args. Each entry line will be splitted by this separator
    #[arg(short, long, default_value = " ")]
    pub args_separator: String,
    /// Print command with resolved args instead of running it
    #[arg(short, long)]
    pub debug: bool,
    /// Reads arguments from file instead of standard input
    #[arg(short = 'f', long, value_name = "FILE")]
    pub args_file: Option<PathBuf>,
    #[command(flatten)]
    pub entries: EntriesOptions,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct EntriesOptions {
    /// Separator between entries. Each entry will be splitted by args_separator to produce cmd args
    #[arg(short, long, default_value = "\n")]
    pub entries_separator: String,

    /// Load all input as single entry
    #[arg(short = '0', long = "single-entry", default_value_t = false)]
    pub single_entry: bool,

    /// Splits input into entries of specified size
    #[arg(short = 's', long = "entries-size", default_value_t = 0)]
    pub entry_size: usize,
}
