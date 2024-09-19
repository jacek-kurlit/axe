use logos::Logos;

use super::tokens::{ArgPlaceholderToken, ArgPlaceholderToken::*, ArgTemplateToken, LexingError};
use super::ArgTemplatePart;
use super::ResolvedArgument;

pub fn resolve_template_args(
    arg_templates: &[String],
) -> Result<Vec<ResolvedArgument>, LexingError> {
    arg_templates
        .iter()
        .map(|a| resolve_arg_template(a.as_str()))
        .collect()
}

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

fn resolve_arg_placeholder(placeholder: &str) -> Result<ArgTemplatePart, LexingError> {
    let lex = ArgPlaceholderToken::lexer(placeholder);
    let tokens = lex.collect::<Result<Vec<ArgPlaceholderToken>, LexingError>>()?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
