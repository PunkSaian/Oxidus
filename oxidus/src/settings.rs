use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use macros::settings;
use toml::{Table, Value};

use crate::overlay::OxidusResult;

static mut SETTINGS: Option<Arc<RwLock<Settings>>> = None;

#[derive(Debug, Clone)]
pub enum EntryValue {
    I32(i32),
    F32(f32),
    String(String),
    Bool(bool),
    Color(i32),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Value(EntryValue, EntryValue),
    Group(HashMap<String, Entry>),
}

pub fn config_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap();
    let home_dir = Path::new(&home);
    home_dir.join(".config/").join("oxidus/")
}

pub fn meta_config_file() -> PathBuf {
    config_dir().join("meta.toml")
}

// config files, selected one

#[derive(Debug)]
pub struct Settings {
    /// value, default
    pub entries: HashMap<String, Entry>,
    pub file_path: PathBuf,
}

impl Settings {
    pub fn init() {
        let mut settings = Settings {
            entries: settings! {
                aimbot {
                    enabled: Bool, false,
                    fov: F32, 30.0
                },
                esp {
                    enabled: Bool, false
                }
            },
            file_path: Self::default_settings_file(),
        };

        fs::create_dir_all(Self::settings_dir()).unwrap();

        if !settings.file_path.exists() {
            settings.save().unwrap();
        }

        settings.load().unwrap();

        unsafe {
            SETTINGS = Some(Arc::new(RwLock::new(settings)));
        }
    }
    #[allow(static_mut_refs)]
    pub fn get() -> Arc<RwLock<Settings>> {
        unsafe { SETTINGS.clone().unwrap() }
    }
    pub fn save(&self) -> OxidusResult<()> {
        let table = self.to_toml_table()?;
        let toml = toml::to_string_pretty(&table)?;
        fs::write(&self.file_path, toml)?;
        Ok(())
    }

    pub fn load(&mut self) -> OxidusResult<()> {
        let contents = fs::read_to_string(&self.file_path)?;
        let loaded = contents.parse::<Table>()?;
        self.merge_toml(&loaded);
        Ok(())
    }

    fn to_toml_table(&self) -> OxidusResult<Table> {
        let mut table = Table::new();
        for (key, value) in &self.entries {
            table.insert(key.clone(), value_to_toml(value)?);
        }
        Ok(table)
    }

    fn merge_toml(&mut self, loaded: &Table) {
        for (key, value) in &mut self.entries {
            if let Some(loaded_value) = loaded.get(key) {
                let merged = merge_entry(value, loaded_value);
                *value = merged;
            }
        }
    }
    pub fn settings_dir() -> PathBuf {
        config_dir().join("settings/")
    }
    pub fn default_settings_file() -> PathBuf {
        Self::settings_dir().join("config.toml")
    }
}

fn value_to_toml(entry: &Entry) -> OxidusResult<Value> {
    match entry {
        Entry::Value(value, ..) => match value {
            EntryValue::I32(v) => Ok(Value::Integer(i64::from(*v))),
            EntryValue::F32(v) => Ok(Value::Float(f64::from(*v))),
            EntryValue::String(v) => Ok(Value::String(v.clone())),
            EntryValue::Bool(v) => Ok(Value::Boolean(*v)),
            EntryValue::Color(v) => Ok(Value::String(format!("#{:06X}", v))),
        },
        Entry::Group(map) => {
            let mut table = Table::new();
            for (key, value) in map {
                table.insert(key.clone(), value_to_toml(value)?);
            }
            Ok(Value::Table(table))
        }
    }
}

fn merge_entry(current: &Entry, loaded: &Value) -> Entry {
    match (current, loaded) {
        (Entry::Value(EntryValue::I32(_), def), Value::Integer(v)) => {
            Entry::Value(EntryValue::I32(*v as i32), def.clone())
        }
        (Entry::Value(EntryValue::F32(_), def), Value::Float(v)) => {
            Entry::Value(EntryValue::F32(*v as f32), def.clone())
        }
        (Entry::Value(EntryValue::String(_), def), Value::String(v)) => {
            Entry::Value(EntryValue::String(v.clone()), def.clone())
        }
        (Entry::Value(EntryValue::Bool(_), def), Value::Boolean(v)) => {
            Entry::Value(EntryValue::Bool(*v), def.clone())
        }
        (Entry::Value(EntryValue::Color(_), def), Value::String(v)) => {
            let hex = v.trim_start_matches('#');
            Entry::Value(
                EntryValue::Color(i32::from_str_radix(hex, 16).unwrap()),
                def.clone(),
            )
        }
        (Entry::Group(cur_map), Value::Table(loaded_table)) => {
            let mut merged_map = HashMap::new();
            for (key, value) in cur_map {
                if let Some(loaded_val) = loaded_table.get(key) {
                    let merged = merge_entry(value, loaded_val);
                    merged_map.insert(key.clone(), merged);
                } else {
                    merged_map.insert(key.clone(), value.clone());
                }
            }
            Entry::Group(merged_map)
        }
        _ => current.clone(),
    }
}

pub fn init_settings() {
    Settings::init();
}
