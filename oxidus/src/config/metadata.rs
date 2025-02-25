use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use toml::Table;

use crate::overlay::OxidusResult;

use super::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub current_config: PathBuf,
}

impl MetaData {
    pub fn meta_config_file() -> PathBuf {
        Config::config_dir().join("meta.toml")
    }

    pub fn load() -> OxidusResult<MetaData> {
        let contents = fs::read_to_string(Self::meta_config_file())?;
        let loaded = contents.parse::<Table>()?;
        let current_config = loaded
            .get("current_config")
            .map_or_else(Config::default_config_file, |v| {
                PathBuf::from(v.as_str().unwrap())
            });
        Ok(MetaData { current_config })
    }
    pub fn save(&self) -> OxidusResult<()> {
        let toml = toml::to_string_pretty(&self)?;
        fs::write(Self::meta_config_file(), toml)?;
        Ok(())
    }
}
