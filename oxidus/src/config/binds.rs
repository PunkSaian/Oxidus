use std::collections::HashMap;

use imgui::Key;

use super::{Config, Entry, EntryValue};

#[derive(Debug, Clone)]
pub struct Bind {
    pub name: String,
    pub keys: Vec<Key>,
    pub diff: HashMap<Vec<String>, EntryValue>,
    pub triggered: bool,
}

impl Bind {
    pub fn apply_overwrite(&self, config: &mut Config) {
        for (path, value) in &self.diff {
            let mut entry = &mut config.settings;
            for key in path {
                match entry.get_mut(key).unwrap() {
                    Entry::Group(group) => entry = group,
                    Entry::Value(_, _, overwrite) => {
                        *overwrite = Some(value.clone());
                        break;
                    }
                }
            }
        }
    }
    pub fn apply(&self, config: &mut Config) {
        for (path, bind_value) in &self.diff {
            let mut entry = &mut config.settings;
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
    let config = Config::get();
    let mut config = config.write().unwrap();
    for (i, bind) in config.binds.clone().iter().enumerate() {
        let triggered = bind.keys.iter().all(|x| ui.is_key_down(*x));
        if triggered == bind.triggered {
            continue;
        }
        bind.apply_overwrite(&mut config);
        config.binds[i].triggered = triggered;
    }
}
