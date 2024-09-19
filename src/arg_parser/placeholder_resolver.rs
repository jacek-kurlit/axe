use super::{ArgTemplatePart, LexingError};
use logos::Logos;

pub fn resolve_arg_placeholder(placeholder: &str) -> Result<ArgTemplatePart, LexingError> {
    let lex = ArgPlaceholderToken::lexer(placeholder);
    let tokens = lex.collect::<Result<Vec<ArgPlaceholderToken>, LexingError>>()?;
    use ArgPlaceholderToken::*;
    match tokens.as_slice() {
        [BraceOpen, Index(index), BraceClose] => Ok(ArgTemplatePart::Index(*index)),
        [BraceOpen, Index(index), Separator(sep), BraceClose] => {
            Ok(ArgTemplatePart::IndexSplit(*index, sep))
        }
        [BraceOpen, Index(index), Separator(sep), Index(index2), BraceClose] => {
            Ok(ArgTemplatePart::IndexSplitIndex(*index, sep, *index2))
        }
        [BraceOpen, Separator(sep), Index(index), BraceClose] => {
            Ok(ArgTemplatePart::SplitIndex(sep, *index))
        }
        [BraceOpen, Separator(sep), BraceClose] => Ok(ArgTemplatePart::Split(sep)),
        [BraceOpen, BraceClose] => Ok(ArgTemplatePart::Empty),
        _ => Err(LexingError::InvalidDefinition),
    }
}

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
enum ArgPlaceholderToken<'a> {
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Index(usize),
    #[regex(r"[^0-9\{}]+", |lex| lex.slice())]
    Separator(&'a str),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn lexer_should_parse_arg_placeholder_with_index_only() {
        let mut lex = ArgPlaceholderToken::lexer("{0}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Index(0))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_arg_placeholder_with_index_and_separator() {
        let mut lex = ArgPlaceholderToken::lexer("{0.}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Index(0))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Separator("."))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_arg_placeholder_with_index_separator_and_index() {
        let mut lex = ArgPlaceholderToken::lexer("{0.1}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Index(0))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Separator("."))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Index(1))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_empty_arg_placeholder() {
        let mut lex = ArgPlaceholderToken::lexer("{}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_empty_arg_placeholder_with_separator() {
        let mut lex = ArgPlaceholderToken::lexer("{.}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Separator("."))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_arg_placeholder_with_separator_and_index() {
        let mut lex = ArgPlaceholderToken::lexer("{.0}");

        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceOpen)));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Separator("."))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::Index(0))));
        assert_eq!(lex.next(), Some(Ok(ArgPlaceholderToken::BraceClose)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn should_parse_arg_placeholders() {
        assert_eq!(
            ArgTemplatePart::Index(0),
            resolve_arg_placeholder("{0}").unwrap()
        );
        assert_eq!(
            ArgTemplatePart::IndexSplit(0, "."),
            resolve_arg_placeholder("{0.}").unwrap()
        );
        assert_eq!(
            ArgTemplatePart::IndexSplitIndex(0, ".", 1),
            resolve_arg_placeholder("{0.1}").unwrap()
        );
        assert_eq!(
            ArgTemplatePart::SplitIndex(".", 0),
            resolve_arg_placeholder("{.0}").unwrap()
        );
        assert_eq!(
            ArgTemplatePart::Split("."),
            resolve_arg_placeholder("{.}").unwrap()
        );
        assert_eq!(
            ArgTemplatePart::Empty,
            resolve_arg_placeholder("{}").unwrap()
        );
    }

    #[test]
    fn should_fail_to_parse_arg_placeholders() {
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("}").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{0").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{0.").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{0.1").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("xxx").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("0.0").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{0}x").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("0{0}").unwrap_err()
        );
        assert_eq!(
            LexingError::InvalidDefinition,
            resolve_arg_placeholder("{{0}}").unwrap_err()
        );
    }
}
