use std::collections::HashMap;

use imgui::Key;
use toml::{value::Array, Table, Value};

use super::{Config, Entry, EntryValue};

#[derive(Debug)]
pub struct Binds {
    pub binds: Vec<Bind>,
}

impl Binds {
    pub fn new() -> Self {
        Self { binds: Vec::new() }
    }

    #[allow(clippy::similar_names)]
    pub fn to_toml_table(&self) -> toml::value::Table {
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
                    Entry::Value(value.clone(), value.clone(), None).to_toml(),
                );
                diffs_table.push(Value::Table(diff_table));
            }
            bind_table.insert("diff".to_owned(), Value::Array(diffs_table));
            bind_table.insert(
                "keys".to_owned(),
                Value::Array(
                    bind.keys
                        .iter()
                        .map(|x| Value::Integer(i64::from(*x as u32)))
                        .collect(),
                ),
            );
            binds_table.insert(bind.name.clone(), Value::Table(bind_table));
        }
        binds_table
    }
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub name: String,
    pub keys: Vec<Key>,
    pub diff: HashMap<Vec<String>, EntryValue>,
    pub triggered: bool,
}

impl Bind {
    pub fn overwrite(&self, config: &mut Config, triggered: bool) {
        for (path, value) in &self.diff {
            let mut entry = &mut config.settings_old;
            for key in path {
                match entry.get_mut(key).unwrap() {
                    Entry::Group(group) => entry = group,
                    Entry::Value(_, _, overwrite) => {
                        if triggered {
                            *overwrite = Some(value.clone());
                        } else {
                            *overwrite = None;
                        }
                        break;
                    }
                }
            }
        }
    }
    pub fn apply(&self, config: &mut Config) {
        for (path, bind_value) in &self.diff {
            let mut entry = &mut config.settings_old;
            for key in path {
                match entry.get_mut(key).unwrap() {
                    Entry::Group(group) => entry = group,
                    Entry::Value(value, _, _) => {
                        *value = bind_value.clone();
                        break;
                    }
                }
            }
        }
    }
}

pub fn run_binds(ui: &imgui::Ui) {
    let mut config = Config::get_write();
    for (i, bind) in config.binds.binds.clone().iter().enumerate() {
        let triggered = bind.keys.iter().all(|x| ui.is_key_down(*x));
        if triggered == bind.triggered {
            continue;
        }
        bind.overwrite(&mut config, triggered);
        config.binds.binds[i].triggered = triggered;
    }
}
