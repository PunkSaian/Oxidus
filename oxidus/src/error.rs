use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxidusError {}

pub type OxidusResult<T = (), E = OxidusError> = Result<T, E>;
