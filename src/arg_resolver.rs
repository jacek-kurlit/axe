use templates_resolver::resolve_template_args;
use thiserror::Error;
use tokens::LexingError;

use std::str::Split;

use crate::cli::Cli;

mod templates_resolver;
mod tokens;

// echo abcd{0}efg{1.0} {2} {}
#[derive(Debug, PartialEq, Eq)]
enum ArgTemplatePart<'a> {
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

//FIXME:template_args may be empty, it means that we should append all args as last argument
pub fn resolve_cmd_args(stdin_entries: Vec<String>, cli: &Cli) -> Vec<Vec<String>> {
    let mut entries = Vec::new();
    //FIXME: handle error
    let args_resolver = ArgumentResolver::new(&cli.args_templates).unwrap();

    for stdin_entry in stdin_entries {
        let input_args = stdin_entry
            .split(&cli.args_separator)
            .collect::<Vec<&str>>();
        //FIXME: handle error
        //we may add flag to choose how to behave on error like:
        //panic and break
        //replace invalid value withempty string
        //ignore failed entry and continue with others
        let entry = args_resolver.resolve(input_args).unwrap();
        entries.push(entry);
    }
    entries
}

type ResolvedArgument<'a> = Vec<ArgTemplatePart<'a>>;

struct ArgumentResolver<'a> {
    resolved_args: Vec<ResolvedArgument<'a>>,
    has_any_placeholder: bool,
}

impl<'a> ArgumentResolver<'a> {
    fn new(arg_templates: &'a [String]) -> Result<ArgumentResolver<'a>, LexingError> {
        let resolved_args = resolve_template_args(arg_templates)?;
        let has_any_placeholder = resolved_args.iter().any(|arg_template| {
            arg_template
                .iter()
                .any(|part| !matches!(part, ArgTemplatePart::FreeText(_)))
        });
        Ok(ArgumentResolver {
            resolved_args,
            has_any_placeholder,
        })
    }

    fn resolve(&self, input_args: Vec<&str>) -> Result<Vec<String>, ResolveError> {
        if !self.has_any_placeholder {
            return Ok(input_args.into_iter().map(|a| a.to_string()).collect());
        }
        let mut result = Vec::new();
        for arg_template in &self.resolved_args {
            let mut resolved = self.resolve_arg_template(arg_template, &input_args)?;
            result.append(&mut resolved);
        }
        Ok(result)
    }

    fn resolve_arg_template(
        &self,
        arg_template: &[ArgTemplatePart],
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
    arg_template: &ArgTemplatePart,
    input_args: &[&str],
) -> Result<Vec<String>, ResolveError> {
    let resolved = match arg_template {
        ArgTemplatePart::Index(idx) => vec![get_input_arg(*idx, input_args)?.to_string()],
        ArgTemplatePart::IndexSplit(idx, split_by) => {
            let input_arg = get_input_arg(*idx, input_args)?;
            input_arg.split(*split_by).map(|s| s.to_string()).collect()
        }
        ArgTemplatePart::IndexSplitIndex(idx, split_by, split_idx) => {
            let input_arg = get_input_arg(*idx, input_args)?;
            let mut splitted = input_arg.split(*split_by);
            vec![get_split_arg(*split_idx, &mut splitted)?.to_string()]
        }
        ArgTemplatePart::SplitIndex(split_by, split_idx) => input_args
            .iter()
            .map(|a| {
                let mut splitted = a.split(*split_by);
                get_split_arg(*split_idx, &mut splitted).map(|s| s.to_string())
            })
            .collect::<Result<Vec<String>, ResolveError>>()?,
        ArgTemplatePart::Split(split_by) => input_args
            .iter()
            .map(|a| a.split(split_by).map(|s| s.to_string()))
            .flat_map(|a| a.into_iter())
            .collect::<Vec<String>>(),
        ArgTemplatePart::Empty => input_args.iter().map(|a| a.to_string()).collect(),
        ArgTemplatePart::FreeText(text) => vec![text.to_string()],
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
