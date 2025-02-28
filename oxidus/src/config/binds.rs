use std::{fmt::Debug, mem::transmute, ptr};

use imgui::Key;
use serde::{Deserialize, Serialize};

use crate::mdbg;

use super::{settings::Settings, Config};

#[derive(Debug, Clone)]
pub struct Binds {
    pub binds: Vec<Bind>,
}

impl Binds {
    pub fn new() -> Self {
        Self { binds: Vec::new() }
    }
    pub fn to_toml_array(&self) -> toml::Value {
        let mut array = toml::value::Array::new();
        for bind in &self.binds {
            let table = toml::value::Table::try_from(bind).unwrap();
            array.push(toml::Value::Table(table));
        }
        toml::Value::Array(array)
    }
    pub fn from_toml_array(value: &toml::Value) -> Self {
        let binds = value.as_array().unwrap();
        let binds = binds
            .iter()
            .map(|bind| bind.clone().try_into().unwrap())
            .collect::<Vec<Bind>>();
        Self { binds }
    }
}

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bind {
    pub name: String,
    keys: Vec<u32>,
    #[serde(skip)]
    pub triggered: bool,
    //TODO(oxy): this saves the whole settings struct, maybe save only the overwrites
    pub overwrites: Settings,
}

impl Bind {
    pub fn new(name: &str, keys: &[Key]) -> Self {
        Self {
            name: name.to_string(),
            keys: keys.iter().map(|key| *key as u32).collect(),
            triggered: false,
            overwrites: Settings::default(),
        }
    }
    pub fn get_keys_string(&self) -> String {
        self.keys
            .iter()
            .map(|key| format!("{:?}", unsafe { transmute::<u32, Key>(*key) }))
            .collect::<Vec<String>>()
            .join(" + ")
    }
}

pub fn run_binds(ui: &mut imgui::Ui) {
    let mut config = Config::get_write();
    let Config {
        settings, binds, ..
    } = &mut *config;
    settings.clear_overwrites();
    for bind in &mut binds.binds {
        bind.triggered = bind
            .keys
            .iter()
            .all(|key| ui.is_key_down(unsafe { *ptr::from_ref(key).cast() }));
        if bind.triggered {
            settings.apply_overwrites(&bind.overwrites);
        }
    }
}
