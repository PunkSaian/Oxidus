use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use binds::Binds;
use entry::{Entry, EntryValue};
use metadata::MetaData;
use settings::Settings;
use toml::{Table, Value};

use crate::overlay::OxidusResult;

pub mod binds;
pub mod entry;
pub mod metadata;
pub mod settings;

static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

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
    pub settings_old: HashMap<String, Entry>,
    pub settings: Settings,
    pub binds: Binds,
    pub meta: MetaData,
    pub binding: Option<(usize, HashMap<String, Entry>)>,
}

impl Config {
    #[deprecated]
    pub fn get_default_entries() -> HashMap<String, Entry> {
        HashMap::new()
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

        let mut config = Config {
            binds: Binds::new(),
            settings_old: Self::get_default_entries(),
            #[deprecated]
            settings: Settings::default(),
            meta,
            binding: None,
        };

        if !config.meta.current_config.exists() {
            config.save().unwrap();
        }

        config.load().unwrap();
        dbg!(&config.settings);

        CONFIG.set(RwLock::new(config)).unwrap();
    }

    pub fn get_read() -> RwLockReadGuard<'static, Config> {
        CONFIG.get().unwrap().read().unwrap()
    }
    pub fn get_write() -> RwLockWriteGuard<'static, Config> {
        CONFIG.get().unwrap().write().unwrap()
    }

    pub fn save(&self) -> OxidusResult<()> {
        let toml = toml::to_string_pretty(&self.settings)?;
        fs::write(&self.meta.current_config, toml)?;

        Ok(())
    }

    pub fn load(&mut self) -> OxidusResult<()> {
        let contents = fs::read_to_string(&self.meta.current_config)?;
        let loaded = contents.parse::<Table>()?;
        self.settings = loaded.get("settings").unwrap().clone().try_into()?;
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
            self.settings_old = Self::get_default_entries();
        }
        self.meta.save()?;
        self.save()?;
        Ok(())
    }

    fn merge_toml(&mut self, loaded: &Table) {
        for (key, value) in &mut self.settings_old {
            if let Some(loaded_value) = loaded.get(key) {
                let merged = value.clone().merge(loaded_value);
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

pub fn init_settings() {
    Config::init();
}
#[deprecated]
#[macro_export]
macro_rules! get_setting_mut {
    ($map:expr, $key:expr => $variant:ident) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::entry::Entry::Value($crate::config::entry::EntryValue::$variant(ref mut value), .., overwrite) = entry else {
            panic!(
                "Invalid entry: expected {} at key '{}'",
                stringify!($variant),
                $key
            );
        };
        if let Some(overwrite) = overwrite{
            let $crate::config::entry::EntryValue::$variant(ref mut value) = overwrite else { unreachable!() };
            value
        } else {
            value
        }
    }};

    ($map:expr, $key:expr, $($rest:tt)*) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::entry::Entry::Group(ref mut next_map) = entry else {
            panic!("Invalid entry: expected Group at key '{}'", $key);
        };
        get_setting_mut!(next_map, $($rest)*)
    }};

}

#[deprecated]
#[macro_export]
macro_rules! get_setting {
    ($map:expr, $key:expr => $variant:ident) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::entry::Entry::Value($crate::config::entry::EntryValue::$variant(ref mut value), .., overwrite) = entry else {
            panic!(
                "Invalid entry: expected {} at key '{}'",
                stringify!($variant),
                $key
            );
        };
        if let Some(overwrite) = overwrite{
            let $crate::config::entry::EntryValue::$variant(ref mut value) = overwrite else { unreachable!() };
            *value
        } else {
            *value
        }
    }};

    ($map:expr, $key:expr, $($rest:tt)*) => {{
        let entry = $map.get_mut($key).unwrap();
        let $crate::config::entry::Entry::Group(ref mut next_map) = entry else {
            panic!("Invalid entry: expected Group at key '{}'", $key);
        };
        get_setting!(next_map, $($rest)*)
    }};

}
