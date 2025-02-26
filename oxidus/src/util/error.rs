use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OxidusError {
    #[error("Hooking {}", .0)]
    Hooking(String),
    #[error("Overlay {}", .0)]
    Overlay(String),
    #[error("Generic {}", .0)]
    Generic(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Image(#[from] ImageError),
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
}

pub type OxidusResult<T = (), E = OxidusError> = Result<T, E>;
