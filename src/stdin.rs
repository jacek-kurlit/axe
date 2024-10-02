use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::error::Result;

use crate::cli::{Cli, EntriesOptions};

pub fn read_entries(cli: &Cli) -> Vec<String> {
    let stdin_lines = read_input_lines(&cli.args_file);
    split_input_lines_into_entries(stdin_lines, &cli.entries, &cli.args_separator)
}

fn read_input_lines(args_file: &Option<PathBuf>) -> Vec<String> {
    let reader: Box<dyn BufRead> = match args_file {
        Some(path) => {
            //FIXME: handle error
            let file = std::fs::File::open(path).expect("Could not open file");
            Box::new(BufReader::new(file))
        }
        None => Box::new(io::stdin().lock()),
    };
    reader
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .unwrap_or_else(|_| {
            panic!(
                "Could not read args from {}",
                args_file
                    .as_ref()
                    .and_then(|p| p.to_str())
                    .unwrap_or("stdin")
            )
        })
}

fn split_input_lines_into_entries(
    stdin_lines: Vec<String>,
    entries: &EntriesOptions,
    args_separator: &str,
) -> Vec<String> {
    match (
        entries.single_entry,
        entries.entry_size,
        entries.entries_separator.as_str(),
    ) {
        (true, _, _) => vec![stdin_lines.join(args_separator)],
        (_, size, _) if size > 0 => split_by_size(stdin_lines, size, args_separator),
        (_, _, "\n") => stdin_lines,
        (_, _, entry_sep) => stdin_lines
            .join("")
            .split(&entry_sep)
            .map(|l| l.to_owned())
            .collect(),
    }
}

//TODO: This method may cause probvlems for large inputs
fn split_by_size(stdin_lines: Vec<String>, size: usize, sep: &str) -> Vec<String> {
    let all_args: Vec<String> = stdin_lines
        .into_iter()
        .flat_map(|l| l.split(sep).map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();
    all_args.chunks(size).map(|c| c.join(sep)).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_stdio_lines_as_input_entries_for_new_line_separator() {
        let stdin_lines = vec!["a b c".to_string(), "d e f".to_string()];
        let entries_options = EntriesOptions {
            single_entry: false,
            entries_separator: "\n".to_string(),
            entry_size: 0,
        };
        let expected = stdin_lines.clone();
        let actual = split_input_lines_into_entries(stdin_lines, &entries_options, " ");
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_stdio_lines_as_input_entries_for_colon_separator() {
        let entries_options = EntriesOptions {
            single_entry: false,
            entries_separator: ";".to_string(),
            entry_size: 0,
        };
        let stdin_lines = vec!["a b c;d e f;".to_string(), "g h i;j k l".to_string()];
        let expected = vec![
            "a b c".to_string(),
            "d e f".to_string(),
            "g h i".to_string(),
            "j k l".to_string(),
        ];
        let actual = split_input_lines_into_entries(stdin_lines.clone(), &entries_options, " ");
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_stdio_lines_as_single_entry() {
        let entries_options = EntriesOptions {
            single_entry: true,
            entries_separator: "\n".to_string(),
            entry_size: 0,
        };
        let stdin_lines = vec!["a,b,c".to_string(), "d,e,f".to_string()];
        let expected = vec!["a,b,c,d,e,f".to_string()];
        let actual = split_input_lines_into_entries(stdin_lines, &entries_options, ",");
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_stdio_lines_as_entries_with_size() {
        let entries_options = EntriesOptions {
            single_entry: false,
            entries_separator: "\n".to_string(),
            entry_size: 2,
        };
        let stdin_lines = vec!["a;b;c".to_string(), "d;e;f;g".to_string()];
        let expected = vec![
            "a;b".to_string(),
            "c;d".to_string(),
            "e;f".to_string(),
            "g".to_string(),
        ];
        let actual = split_input_lines_into_entries(stdin_lines, &entries_options, ";");
        assert_eq!(expected, actual);
    }
}
