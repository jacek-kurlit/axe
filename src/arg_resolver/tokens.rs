use std::num::ParseIntError;

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
pub enum ArgTemplateToken {
    #[regex(r"\{[^{}]*\}", priority = 2)]
    ArgPlaceholder,
    #[regex(r"[^{}\\]+", priority = 0)]
    FreeText,
    #[regex(r"\\\{*[^{}]*}*", priority = 1)]
    EscapedText,
}

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
pub enum ArgPlaceholderToken<'a> {
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Index(usize),
    #[regex(r"[^0-9\{}]+", |lex| lex.slice())]
    Separator(&'a str),
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    InvalidInteger(String),
    #[default]
    InvalidDefinition,
}

impl From<ParseIntError> for LexingError {
    fn from(err: ParseIntError) -> Self {
        use std::num::IntErrorKind::*;
        match err.kind() {
            PosOverflow | NegOverflow => LexingError::InvalidInteger("overflow error".to_owned()),
            _ => LexingError::InvalidInteger("other error".to_owned()),
        }
    }
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

    use super::ArgTemplateToken;

    #[test]
    fn lexer_should_parse_arg_template_with_placeholders() {
        let mut lex = ArgTemplateToken::lexer("{}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("{0}{1#}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{1#}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("{0.0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0.0}");
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_arg_template_with_free_text() {
        let mut lex = ArgTemplateToken::lexer("free.0{0}1_text");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "free.0");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "1_text");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("{0}__{1} {2}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "__");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{1}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), " ");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{2}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("free text without placeholders");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "free text without placeholders");
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_fail_to_parse_arg_template_with_invalid_placeholders() {
        let mut lex = ArgTemplateToken::lexer("free{");

        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "free");
        assert_eq!(lex.next(), Some(Err(LexingError::InvalidDefinition)));
        assert_eq!(lex.slice(), "{");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("free}");

        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "free");
        assert_eq!(lex.next(), Some(Err(LexingError::InvalidDefinition)));
        assert_eq!(lex.slice(), "}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer("{{0}}");
        assert_eq!(lex.next(), Some(Err(LexingError::InvalidDefinition)));
        assert_eq!(lex.slice(), "{");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.next(), Some(Err(LexingError::InvalidDefinition)));
        assert_eq!(lex.slice(), "}");
    }

    #[test]
    fn lexer_should_parse_arg_tmplate_placeholder_even_when_value_is_invalid() {
        let mut lex = ArgTemplateToken::lexer("{abcd.0}{1.1.1}{0.0#}{_0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{abcd.0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{1.1.1}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0.0#}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{_0}");
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn lexer_should_parse_escaped_characters_in_arg_templates() {
        let mut lex = ArgTemplateToken::lexer(r"free\{");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), r"free");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"free\}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), "free");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"\{0}\{1\}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{1\}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"\\{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\\");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{0}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"\{0abc{0}\}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{0abc");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), r"{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"\{{0}}{1}\d{2}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{{0}}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{1}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\d");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), "{2}");
        assert_eq!(lex.next(), None);

        let mut lex = ArgTemplateToken::lexer(r"\{ {0} \}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\{ ");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::ArgPlaceholder)));
        assert_eq!(lex.slice(), r"{0}");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::FreeText)));
        assert_eq!(lex.slice(), " ");
        assert_eq!(lex.next(), Some(Ok(ArgTemplateToken::EscapedText)));
        assert_eq!(lex.slice(), r"\}");
        assert_eq!(lex.next(), None);
    }
}
