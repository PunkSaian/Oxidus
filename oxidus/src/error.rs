use std::fmt::Display;

use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub struct OxidusError {
    #[source]
    pub r#type: OxidusErrorType,
    pub context_stack: Vec<ContextFrame>,
}

impl OxidusError {
    pub const fn new(r#type: OxidusErrorType) -> Self {
        Self {
            r#type,
            context_stack: Vec::new(),
        }
    }
}

impl Display for OxidusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stack = self.context_stack.iter().fold(String::new(), |mut acc, x| {
            if !acc.is_empty() {
                acc.push('\n');
            }
            acc.push_str(&x.to_string());
            acc
        });
        write!(f, "{}\n{}", self.r#type, stack)
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct ContextFrame {
    file: String,
    line: String,
}

impl Display for ContextFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

#[derive(Error, Debug)]
pub enum OxidusErrorType {
    #[error("Hooking {}", .0)]
    Hooking(String),
    #[error("Generic {}", .0)]
    Generic(String),
}

pub type OxidusResult<T = (), E = OxidusError> = Result<T, E>;

impl<T> From<OxidusErrorType> for OxidusResult<T> {
    fn from(val: OxidusErrorType) -> Self {
        Err(OxidusError::new(val))
    }
}
