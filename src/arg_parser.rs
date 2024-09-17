use template_resolver::resolve_arg_template;
use thiserror::Error;

use std::{num::ParseIntError, str::Split};

mod placeholder_resolver;
mod template_resolver;

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedArgumentPart<'a> {
    //{0}
    Index(usize),
    //{0.}
    IndexSplit(usize, &'a str),
    //{0.0}
    IndexSplitIndex(usize, &'a str, usize),
    //{.0}
    SplitIndex(&'a str, usize),
    //{.}
    Split(&'a str),
    //{}
    Empty,
    //abcd
    FreeText(&'a str),
}

#[derive(Error, Debug)]
pub enum ResolveError {
    //TODO: this needs better error handling because we don't display which arg failed
    //so maybe anyhow will be better so we can keep adding context?
    #[error("Index {0} is out of bounds")]
    InvalidIndex(usize),
    #[error("unknown data store error")]
    Other,
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

    pub fn resolve(&self, input_args: Vec<&str>) -> Result<Vec<String>, ResolveError> {
        let mut result = Vec::new();
        for arg_template in &self.resolved_args {
            let mut resolved = self.resolve_arg_template(arg_template, &input_args)?;
            result.append(&mut resolved);
        }
        Ok(result)
    }

    fn resolve_arg_template(
        &self,
        arg_template: &[ResolvedArgumentPart],
        input_args: &[&str],
    ) -> Result<Vec<String>, ResolveError> {
        let mut resolved = Vec::new();
        for part in arg_template {
            let single_part = resolve_single_arg_part(part, input_args)?;
            resolved = multiply_args_parts(resolved, single_part);
        }
        Ok(resolved)
    }
}

fn resolve_single_arg_part(
    arg_template: &ResolvedArgumentPart,
    input_args: &[&str],
) -> Result<Vec<String>, ResolveError> {
    let resolved = match arg_template {
        ResolvedArgumentPart::Index(idx) => vec![get_input_arg(*idx, input_args)?.to_string()],
        ResolvedArgumentPart::IndexSplit(idx, split_by) => {
            let input_arg = get_input_arg(*idx, input_args)?;
            input_arg.split(*split_by).map(|s| s.to_string()).collect()
        }
        ResolvedArgumentPart::IndexSplitIndex(idx, split_by, split_idx) => {
            let input_arg = get_input_arg(*idx, input_args)?;
            let mut splitted = input_arg.split(*split_by);
            vec![get_split_arg(*split_idx, &mut splitted)?.to_string()]
        }
        ResolvedArgumentPart::SplitIndex(split_by, split_idx) => input_args
            .iter()
            .map(|a| {
                let mut splitted = a.split(*split_by);
                get_split_arg(*split_idx, &mut splitted).map(|s| s.to_string())
            })
            .collect::<Result<Vec<String>, ResolveError>>()?,
        ResolvedArgumentPart::Split(split_by) => input_args
            .iter()
            .map(|a| a.split(split_by).map(|s| s.to_string()))
            .flat_map(|a| a.into_iter())
            .collect::<Vec<String>>(),
        ResolvedArgumentPart::Empty => input_args.iter().map(|a| a.to_string()).collect(),
        ResolvedArgumentPart::FreeText(text) => vec![text.to_string()],
    };
    Ok(resolved)
}

fn get_input_arg<'a>(idx: usize, input_args: &'a [&'a str]) -> Result<&'a str, ResolveError> {
    input_args
        .get(idx)
        .copied()
        .ok_or(ResolveError::InvalidIndex(idx))
}

fn get_split_arg<'a>(
    idx: usize,
    splitted: &'a mut Split<&'a str>,
) -> Result<&'a str, ResolveError> {
    splitted.nth(idx).ok_or(ResolveError::InvalidIndex(idx))
}

//This performs args multiplication for example
//[a,b] * [c] -> [ac,bc]
//[a] * [b] -> [ab]
//[a,b] * [c,d] -> [ac,ad,bc,bd]
fn multiply_args_parts(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    let mut result = Vec::with_capacity(a.len() * b.len());
    for a_part in a {
        for b_part in b.clone() {
            result.push(format!("{}{}", a_part, b_part));
        }
    }

    result
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
