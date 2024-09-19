use std::io::{self, BufRead};

pub fn read_stdin_entries(entries_separator: &str) -> Vec<String> {
    let stdin_lines = read_input_lines();
    split_input_lines_into_entries(stdin_lines, entries_separator)
}

fn read_input_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line.expect("Could not read line from standard in"));
    }
    lines
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
