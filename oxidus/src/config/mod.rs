use std::{
    fs,
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use binds::Binds;
use metadata::MetaData;
use settings::Settings;
use toml::{Table, Value};

use crate::overlay::OxidusResult;

pub mod binds;
pub mod metadata;
pub mod settings;

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    pub settings: Settings,
    pub binds: Binds,
    pub meta: MetaData,
    pub binding: Option<(usize, Settings)>,
}

impl Config {
    pub fn init() {
        fs::create_dir_all(Self::configs_dir()).unwrap();
        let meta = if MetaData::meta_config_file().exists() {
            MetaData::load().unwrap()
        } else {
            let meta = MetaData {
                current_config: Self::default_config_file(),
            };
            meta.save().unwrap();
            meta
        };

        let mut config = Config {
            binds: Binds::new(),
            settings: Settings::default(),
            meta,
            binding: None,
        };

        let mut default = Settings::default();
        default.aimbot.enabled.overwrite = Some(true);
        config
            .binds
            .binds
            .push(binds::Bind::new("test", &[imgui::Key::Tab], default));

        if !config.meta.current_config.exists() {
            config.save().unwrap();
        }

        config.load().unwrap();

        CONFIG.set(RwLock::new(config)).unwrap();
    }

    pub fn get_read() -> RwLockReadGuard<'static, Config> {
        CONFIG.get().unwrap().try_read().unwrap()
    }
    pub fn get_write() -> RwLockWriteGuard<'static, Config> {
        CONFIG.get().unwrap().try_write().unwrap()
    }

    pub fn save(&self) -> OxidusResult<()> {
        let mut toml = toml::Table::new();
        //cast self.string NOT self.settings_old to toml table
        let settings = Table::try_from(self.settings.clone())?;

        toml.insert("settings".to_string(), Value::Table(settings));

        //TODO(oxy): save binds
        toml.insert("binds".to_string(), Value::Table(toml::Table::new()));

        fs::write(&self.meta.current_config, toml::to_string_pretty(&toml)?)?;

        Ok(())
    }

    pub fn load(&mut self) -> OxidusResult<()> {
        let contents = fs::read_to_string(&self.meta.current_config)?;
        let loaded = contents.parse::<Table>()?;
        self.settings = Settings::default();
        self.settings
            .merge(&loaded.get("settings").unwrap().clone().try_into()?);
        Ok(())
    }

    pub fn switch_config(&mut self, file: &PathBuf) -> OxidusResult<()> {
        self.save()?;
        self.meta.current_config.clone_from(file);
        self.load()?;
        self.meta.save()?;
        Ok(())
    }

    pub fn delete(file: &PathBuf) -> OxidusResult<()> {
        fs::remove_file(file)?;
        Ok(())
    }

    pub fn create(&mut self, file_name: &str, copy: bool) -> OxidusResult<()> {
        let file_name_with_ext = file_name.to_owned() + ".toml";
        let file = Self::configs_dir().join(file_name_with_ext);
        self.save()?;
        self.meta.current_config.clone_from(&file);
        if !copy {
            self.settings = Settings::default();
        }
        self.meta.save()?;
        self.save()?;
        Ok(())
    }

    pub fn config_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap();
        let home_dir = Path::new(&home);
        home_dir.join(".config/").join("oxidus/")
    }

    pub fn configs_dir() -> PathBuf {
        Self::config_dir().join("configs/")
    }
    pub fn default_config_file() -> PathBuf {
        Self::configs_dir().join("default.toml")
    }
    pub fn get_config_files() -> OxidusResult<Vec<PathBuf>> {
        let mut files = vec![];
        for entry in fs::read_dir(Self::configs_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
        Ok(files)
    }
}

pub fn init_settings() {
    Config::init();
}
