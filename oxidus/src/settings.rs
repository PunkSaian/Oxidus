use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use macros::settings;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub current_config: PathBuf,
}

impl MetaData {
    pub fn meta_config_file() -> PathBuf {
        Settings::config_dir().join("meta.toml")
    }

    pub fn load() -> OxidusResult<MetaData> {
        let contents = fs::read_to_string(Self::meta_config_file())?;
        let loaded = contents.parse::<Table>()?;
        let current_config = loaded
            .get("current_config")
            .map_or_else(Settings::default_config_file, |v| {
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

#[derive(Debug)]
pub struct Settings {
    /// value, default
    pub config: HashMap<String, Entry>,
    pub meta: MetaData,
}

impl Settings {
    pub fn get_default_entries() -> HashMap<String, Entry> {
        settings! {
            aimbot {
                enabled: Bool, false,
                fov: F32, 30.0
            },
            esp {
                enabled: Bool, false
            }
        }
    }
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

        let mut settings = Settings {
            config: Self::get_default_entries(),
            meta,
        };

        if !settings.meta.current_config.exists() {
            settings.save_config().unwrap();
        }

        settings.load_config().unwrap();

        unsafe {
            SETTINGS = Some(Arc::new(RwLock::new(settings)));
        }
    }

    #[allow(static_mut_refs)]
    pub fn get() -> Arc<RwLock<Settings>> {
        unsafe { SETTINGS.clone().unwrap() }
    }
    pub fn save_config(&self) -> OxidusResult<()> {
        let table = self.to_toml_table()?;
        let toml = toml::to_string_pretty(&table)?;
        fs::write(&self.meta.current_config, toml)?;
        Ok(())
    }

    pub fn load_config(&mut self) -> OxidusResult<()> {
        let contents = fs::read_to_string(&self.meta.current_config)?;
        let loaded = contents.parse::<Table>()?;
        self.config = Self::get_default_entries();
        self.merge_toml(&loaded);
        Ok(())
    }

    pub fn switch_config(&mut self, file: &PathBuf) -> OxidusResult<()> {
        self.save_config()?;
        self.meta.current_config.clone_from(file);
        self.load_config()?;
        self.meta.save()?;
        Ok(())
    }

    pub fn delete_config(file: &PathBuf) -> OxidusResult<()> {
        fs::remove_file(file)?;
        Ok(())
    }

    pub fn create_new(&mut self, file_name: &str, copy: bool) -> OxidusResult<()> {
        let file_name_with_ext = file_name.to_owned() + ".toml";
        let file = Self::configs_dir().join(file_name_with_ext);
        self.save_config()?;
        self.meta.current_config.clone_from(&file);
        if !copy {
            self.config = Self::get_default_entries();
        }
        self.meta.save()?;
        self.save_config()?;
        Ok(())
    }

    fn to_toml_table(&self) -> OxidusResult<Table> {
        let mut table = Table::new();
        for (key, value) in &self.config {
            table.insert(key.clone(), value_to_toml(value)?);
        }
        Ok(table)
    }

    fn merge_toml(&mut self, loaded: &Table) {
        for (key, value) in &mut self.config {
            if let Some(loaded_value) = loaded.get(key) {
                let merged = merge_entry(value, loaded_value);
                *value = merged;
            }
        }
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

fn value_to_toml(entry: &Entry) -> OxidusResult<Value> {
    match entry {
        Entry::Value(value, ..) => match value {
            EntryValue::I32(v) => Ok(Value::Integer(i64::from(*v))),
            EntryValue::F32(v) => Ok(Value::Float(f64::from(*v))),
            EntryValue::String(v) => Ok(Value::String(v.clone())),
            EntryValue::Bool(v) => Ok(Value::Boolean(*v)),
            EntryValue::Color(v) => Ok(Value::String(format!("#{v:06X}"))),
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

#[macro_export]
macro_rules! get_entry_mut {
    ($map:expr, $key:expr => $variant:ident) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::settings::Entry::Value($crate::settings::EntryValue::$variant(ref mut value), ..) = entry else {
            panic!(
                "Invalid entry: expected {} at key '{}'", 
                stringify!($variant), 
                $key
            );
        };
        value
    }};
    
    ($map:expr, $key:expr, $($rest:tt)*) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::settings::Entry::Group(ref mut next_map) = entry else {
            panic!("Invalid entry: expected Group at key '{}'", $key);
        };
        get_entry_mut!(next_map, $($rest)*)
    }};

}
