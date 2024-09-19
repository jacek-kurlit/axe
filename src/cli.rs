use clap::{arg, command};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = Some("Run command for each entry of arguments"))]
pub struct Cli {
    #[arg(default_value = "echo")]
    pub cmd: String,
    pub args_templates: Vec<String>,
    #[arg(short, long, default_value = " ")]
    pub args_separator: String,
    #[arg(short, long, default_value = "\n")]
    pub entries_separator: String,
}
