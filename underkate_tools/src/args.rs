use crate::common::ResourceType;
use proc_macro::TokenStream;
use syn::{Lit, LitStr};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args {
    pub path: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListArgs {
    pub resource_specs: Vec<(String, ResourceType)>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    SynError(syn::Error),
    InvalidLiteralKind,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SynError(error) => write!(f, "Expected path; {}", error),
            Self::InvalidLiteralKind => write!(f, "Expected string path literal"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_args(tokens: TokenStream) -> Result<Args, ParseError> {
    let path_lit: Lit = syn::parse(tokens).map_err(ParseError::SynError)?;
    let path_str = match path_lit {
        Lit::Str(lit_str) => lit_str.value(),
        _ => return Err(ParseError::InvalidLiteralKind),
    };
    Ok(Args { path: path_str })
}

pub fn parse_list_args(tokens: TokenStream) -> ListArgs {
    #[derive(Debug)]
    enum State {
        ReadingPath,
        ReadingColon {
            path: String,
        },
        ReadingType {
            path: String,
        },
        ReadingDelimiter {
            path: String,
            resource_type: ResourceType,
        },
    }

    let mut resource_specs = Vec::new();

    let mut current_state = State::ReadingPath;
    for token in tokens {
        match current_state {
            State::ReadingPath => {
                let path_lit: LitStr = syn::parse(token.clone().into())
                    .expect(&format!("Expected a resource path, got {:?}", &token));
                let path = path_lit.value();
                current_state = State::ReadingColon { path };
            }
            State::ReadingColon { path } => {
                if token.to_string() == ":" {
                    current_state = State::ReadingType { path };
                } else {
                    panic!("Expected a colon (':'), got {:?}", token);
                }
            }
            State::ReadingType { path } => {
                let resource_type = match &token.to_string() as &str {
                    "texture" => ResourceType::Texture,
                    "pass_map" => ResourceType::PassMap,
                    "rust_script" => ResourceType::RustScript,
                    "room" => ResourceType::Room,
                    x => panic!("Invalid resource type {:?}", x),
                };
                current_state = State::ReadingDelimiter {
                    path,
                    resource_type,
                };
            }
            State::ReadingDelimiter {
                path,
                resource_type,
            } => {
                if token.to_string() == ";" {
                    resource_specs.push((path, resource_type));
                    current_state = State::ReadingPath;
                } else {
                    panic!("Expected a semicolon (';'), got {:?}", token);
                }
            }
        }
    }

    ListArgs { resource_specs }
}
