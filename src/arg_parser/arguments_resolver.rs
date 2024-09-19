use logos::Logos;

use super::{
    placeholder_resolver::resolve_arg_placeholder, ArgTemplatePart, LexingError, ResolvedArgument,
};

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexingError)]
enum ArgTemplateToken {
    #[regex(r"\{[^{}]*\}", priority = 2)]
    ArgPlaceholder,
    #[regex(r"[^{}\\]+", priority = 0)]
    FreeText,
    #[regex(r"\\\{*[^{}]*}*", priority = 1)]
    EscapedText,
}

pub fn resolve_template_args(
    arg_templates: &Vec<String>,
) -> Result<Vec<ResolvedArgument>, LexingError> {
    let mut result = Vec::new();

    for arg_template in arg_templates {
        let resolved = resolve_arg_template(arg_template)?;
        result.push(resolved);
    }

    Ok(result)
}

//TODO: add integration test
fn resolve_arg_template(arg_template: &str) -> Result<ResolvedArgument, LexingError> {
    let mut lex = ArgTemplateToken::lexer(arg_template);
    let mut resolved = Vec::new();
    while let Some(token) = lex.next() {
        match token? {
            ArgTemplateToken::ArgPlaceholder => {
                let ra = resolve_arg_placeholder(lex.slice())?;
                resolved.push(ra);
            }
            ArgTemplateToken::FreeText | ArgTemplateToken::EscapedText => {
                let ra = ArgTemplatePart::FreeText(lex.slice());
                resolved.push(ra)
            }
        }
    }
    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use super::{ArgTemplateToken, LexingError};

    #[test]
    fn parse_arg_placeholder() {
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
    fn parse_arg_placeholder_with_free_text() {
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
    fn parse_failes_when_placeholder_is_invalid() {
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
    fn parse_arg_placeholder_even_when_placeholder_value_is_invalid() {
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
    fn parse_escaped_characters() {
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
    }
}
