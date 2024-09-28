use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::error::Result;

pub fn read_entries(args_file: &Option<PathBuf>, entries_separator: &str) -> Vec<String> {
    let stdin_lines = read_input_lines(args_file);
    split_input_lines_into_entries(stdin_lines, entries_separator)
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
    entries_separator: &str,
) -> Vec<String> {
    if entries_separator == "\n" {
        return stdin_lines;
    }
    stdin_lines
        .join("")
        .split(entries_separator)
        .map(|l| l.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_parse_stdio_lines_as_input_entries_for_new_line_separator() {
        let stdin_lines = vec!["a b c".to_string(), "d e f".to_string()];
        let expected = stdin_lines.clone();
        let actual = split_input_lines_into_entries(stdin_lines, "\n");
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
        let actual = split_input_lines_into_entries(stdin_lines.clone(), ";");
        assert_eq!(expected, actual);
    }
}
