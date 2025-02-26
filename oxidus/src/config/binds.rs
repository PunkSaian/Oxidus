use std::{fmt::Debug, ptr};

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
}

// Updated Bind struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bind {
    pub name: String,
    //TODO(oxy): fix this
    keys: Vec<u32>,
    #[serde(skip)]
    pub triggered: bool,
    pub overwrites: Settings,
}

impl Bind {
    pub fn new(name: &str, keys: &[Key], overwrites: Settings) -> Self {
        Self {
            name: name.to_string(),
            keys: keys.iter().map(|key| *key as u32).collect(),
            triggered: false,
            overwrites,
        }
    }
}

pub fn run_binds(ui: &mut imgui::Ui) {
    let mut config = Config::get_write();
    let Config {
        settings, binds, ..
    } = &mut *config;
    mdbg!(settings);
    settings.clear_overwrites();
    for bind in &mut binds.binds {
        bind.triggered = bind
            .keys
            .iter()
            .all(|key| ui.is_key_down(unsafe { *ptr::from_ref(key).cast() }));
        mdbg!(bind.triggered);
        if bind.triggered {
            settings.apply_overwrites(&bind.overwrites);
        }
    }
}
