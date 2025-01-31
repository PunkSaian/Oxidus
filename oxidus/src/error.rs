use std::fmt::Display;

use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Error, Debug)]
pub struct OxidusError {
    #[source]
    pub r#type: OxidusErrorType,
    pub context_stack: Vec<ContextFrame>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ContextFrame {
    file: String,
    line: String,
    module: String,
}

#[derive(Error, Debug)]
pub enum OxidusErrorType {}

impl Display for OxidusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub type OxidusResult<T = (), E = OxidusError> = Result<T, E>;
