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

pub struct ArgumentResolver<'a> {
    resolved_args: Vec<ResolvedArgument<'a>>,
}

impl<'a> ArgumentResolver<'a> {
    pub fn new(arg_templates: &'a Vec<String>) -> Result<ArgumentResolver<'a>, LexingError> {
        let resolved_args = resolve_template_args(arg_templates)?;
        Ok(ArgumentResolver { resolved_args })
    }
    
    //TODO: return Result, for example someone could provide a wrong index
    //on the other hand, we could add option to ignore errors
    //which means parsing in either strict or non-strict mode
    pub fn resolve(&self, input_args: Vec<&str>) -> Vec<String> {
        let mut result = Vec::new();
        for resolved_arg in &self.resolved_args {
            for part in resolved_arg {
                match part {
                    ResolvedArgumentPart::Index(index) => {
                        let index = *index as usize;
                        if index < input_args.len() {
                            arg.push_str(input_args[index]);
                        }
                    }
                    ResolvedArgumentPart::IndexSplit(index, split) => {
                        let index = *index as usize;
                        if index < input_args.len() {
                            let parts = input_args[index].split(split).collect::<Vec<&str>>();
                            if !parts.is_empty() {
                                arg.push_str(parts[0]);
                            }
                        }
                    }
                    ResolvedArgumentPart::IndexSplitIndex(index, split, split_index) => {
                        let index = *index as usize;
                        if index < input_args.len() {
                            let parts = input_args[index].split(split).collect::<Vec<&str>>();
                            if *split_index as usize < parts.len() {
                                arg.push_str(parts[*split_index as usize]);
                            }
                        }
                    }
                    ResolvedArgumentPart::SplitIndex(split, split_index) => {
                        let parts = split.split(split).collect::<Vec<&str>>();
                        if *split_index as usize < parts.len() {
                            arg.push_str(parts[*split_index as usize]);
                        }
                    }
                    ResolvedArgumentPart::Split(split) => {
                        let parts = split.split(split).collect::<Vec<&str>>();
                        if !parts.is_empty() {
                            arg.push_str(parts[0]);
                        }
                    }
                    ResolvedArgumentPart::Empty => {}
                    ResolvedArgumentPart::FreeText(text) => {
                        arg.push_str(text);
                    }
                }
            }
            result.push(arg);
        }
        result
    }
}

fn resolve_template_args(
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
