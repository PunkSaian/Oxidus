use std::{
    collections::HashMap,
    fs,
    mem::transmute,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use imgui::Key;
use macros::settings;
use serde::{Deserialize, Serialize};
use toml::{value::Array, Table, Value};

use crate::overlay::OxidusResult;

pub mod binds;

static mut SETTINGS: Option<Arc<RwLock<Config>>> = None;

#[derive(Debug, Clone, PartialEq)]
pub enum EntryValue {
    I32(i32),
    F32(f32),
    String(String),
    Bool(bool),
    Color(i32),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Value(EntryValue, EntryValue, Option<EntryValue>),
    Group(HashMap<String, Entry>),
}

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

pub fn diff_settings(
    old: &HashMap<String, Entry>,
    new: &HashMap<String, Entry>,
) -> HashMap<Vec<String>, EntryValue> {
    let mut diff = HashMap::<Vec<String>, EntryValue>::new();
    for (key, new_entry) in new {
        let old_entry_value = old.get(key).unwrap();
        match (old_entry_value, new_entry) {
            (Entry::Value(old_entry_value, ..), Entry::Value(new_entry_value, ..))
                if old_entry_value != new_entry_value =>
            {
                diff.insert(vec![key.clone()], new_entry_value.clone());
            }
            (Entry::Group(old_map), Entry::Group(new_map)) => {
                let inner_diff = diff_settings(old_map, new_map);
                for (mut path, entry) in inner_diff {
                    path.insert(0, key.clone());
                    diff.insert(path, entry);
                }
            }
            _ => {}
        }
    }
    diff
}

#[derive(Debug)]
pub struct Config {
    pub settings: HashMap<String, Entry>,
    pub binds: Vec<binds::Bind>,
    pub meta: MetaData,
    pub binding: Option<(usize, HashMap<String, Entry>)>,
}

impl Config {
    pub fn get_default_entries() -> HashMap<String, Entry> {
        settings! {
            aimbot {
                enabled: Bool, false,
                fov: F32, 30.0
            },
            esp {
                enabled: Bool, false
            }
            visual {
                fov: F32, 100.0
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

        let mut settings = Config {
            binds: Vec::new(),
            settings: Self::get_default_entries(),
            meta,
            binding: None,
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
    pub fn get() -> Arc<RwLock<Config>> {
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
        self.settings = Self::get_default_entries();
        self.merge_toml(loaded.get("settings").unwrap().as_table().unwrap());
        //load binds
        if let Some(Some(binds_table)) = loaded.get("binds").map(|x| x.as_table()) {
            for (name, bind_table) in binds_table {
                let mut keys = Vec::new();
                let mut diff = HashMap::new();
                let bind_table = bind_table.as_table().unwrap();
                for (key, value) in bind_table {
                    match key.as_str() {
                        "diff" => {
                            let diff_table = value.as_array().unwrap();
                            for diff_entry in diff_table {
                                let diff_entry = diff_entry.as_table().unwrap();
                                let path = diff_entry
                                    .get("path")
                                    .unwrap()
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|x| x.as_str().unwrap().to_owned())
                                    .collect();

                                let value = diff_entry.get("value").unwrap();
                                diff.insert(path, value_from_toml(value));
                            }
                        }
                        "keys" => {
                            keys = value
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(|x| unsafe {
                                    transmute::<u32, Key>(x.as_integer().unwrap() as u32)
                                })
                                .collect();
                        }
                        _ => {}
                    }
                }
                self.binds.push(binds::Bind {
                    name: name.clone(),
                    keys,
                    diff,
                    triggered: false,
                });
            }
        }
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
            self.settings = Self::get_default_entries();
        }
        self.meta.save()?;
        self.save_config()?;
        Ok(())
    }

    fn to_toml_table(&self) -> OxidusResult<Table> {
        let mut settings = Table::new();
        for (key, value) in &self.settings {
            settings.insert(key.clone(), value_to_toml(value)?);
        }
        let mut binds_table = Table::new();
        for bind in &self.binds {
            let mut bind_table = Table::new();
            let mut diffs_table = Array::new();
            for (path, value) in &bind.diff {
                let mut diff_table = Table::new();
                diff_table.insert(
                    "path".to_owned(),
                    Value::Array(path.iter().map(|x| Value::String(x.clone())).collect()),
                );
                diff_table.insert(
                    "value".to_owned(),
                    value_to_toml(&Entry::Value(value.clone(), value.clone(), None))?,
                );
                diffs_table.push(Value::Table(diff_table));
            }
            bind_table.insert("diff".to_owned(), Value::Array(diffs_table));
            bind_table.insert(
                "keys".to_owned(),
                Value::Array(
                    bind.keys
                        .iter()
                        .map(|x| Value::Integer((*x as u32) as i64))
                        .collect(),
                ),
            );
            binds_table.insert(bind.name.clone(), Value::Table(bind_table));
        }
        let mut table = Table::new();
        table.insert("settings".to_owned(), Value::Table(settings));
        table.insert("binds".to_owned(), Value::Table(binds_table));
        Ok(table)
    }

    fn merge_toml(&mut self, loaded: &Table) {
        for (key, value) in &mut self.settings {
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

fn value_from_toml(value: &Value) -> EntryValue {
    match value {
        Value::Integer(v) => EntryValue::I32(*v as i32),
        Value::Float(v) => EntryValue::F32(*v as f32),
        Value::String(v) => EntryValue::String(v.clone()),
        Value::Boolean(v) => EntryValue::Bool(*v),
        _ => unreachable!(),
    }
}

fn merge_entry(current: &Entry, loaded: &Value) -> Entry {
    match (current, loaded) {
        (Entry::Value(EntryValue::I32(_), def, _), Value::Integer(v)) => {
            Entry::Value(EntryValue::I32(*v as i32), def.clone(), None)
        }
        (Entry::Value(EntryValue::F32(_), def, _), Value::Float(v)) => {
            Entry::Value(EntryValue::F32(*v as f32), def.clone(), None)
        }
        (Entry::Value(EntryValue::String(_), def, _), Value::String(v)) => {
            Entry::Value(EntryValue::String(v.clone()), def.clone(), None)
        }
        (Entry::Value(EntryValue::Bool(_), def, _), Value::Boolean(v)) => {
            Entry::Value(EntryValue::Bool(*v), def.clone(), None)
        }
        (Entry::Value(EntryValue::Color(_), def, _), Value::String(v)) => {
            let hex = v.trim_start_matches('#');
            Entry::Value(
                EntryValue::Color(i32::from_str_radix(hex, 16).unwrap()),
                def.clone(),
                None,
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
    Config::init();
}

#[macro_export]
macro_rules! get_setting_mut {
    ($map:expr, $key:expr => $variant:ident) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::Entry::Value($crate::config::EntryValue::$variant(ref mut value), .., overwrite) = entry else {
            panic!(
                "Invalid entry: expected {} at key '{}'",
                stringify!($variant),
                $key
            );
        };
        if let Some(overwrite) = overwrite{
            let $crate::config::EntryValue::$variant(ref mut value) = overwrite else { unreachable!() };
            value
        } else {
            value
        }
    }};

    ($map:expr, $key:expr, $($rest:tt)*) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::Entry::Group(ref mut next_map) = entry else {
            panic!("Invalid entry: expected Group at key '{}'", $key);
        };
        get_setting_mut!(next_map, $($rest)*)
    }};

}
