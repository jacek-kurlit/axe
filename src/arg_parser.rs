use template_resolver::resolve_arg_template;

use std::num::ParseIntError;

mod placeholder_resolver;
mod template_resolver;

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedArgumentPart<'a> {
    //{0}
    Index(u8),
    //{0.}
    IndexSplit(u8, &'a str),
    //{0.0}
    IndexSplitIndex(u8, &'a str, u8),
    //{.0}
    SplitIndex(&'a str, u8),
    //{.}
    Split(&'a str),
    //{}
    Empty,
    //abcd
    FreeText(&'a str),
}

pub type ResolvedArgument<'a> = Vec<ResolvedArgumentPart<'a>>;

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
